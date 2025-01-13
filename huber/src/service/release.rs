use std::collections::HashMap;
use std::fs::{read_dir, read_link, remove_dir_all, remove_file, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::{env, fs};

use anyhow::anyhow;
use async_trait::async_trait;
use compress_tools::{uncompress_archive, Ownership};
use fs_extra::move_items;
use huber_common::model::config::{Config, ConfigFieldConvertTrait, ConfigPath};
use huber_common::model::package::{GithubPackage, Package, PackageDetailType, PackageSource};
use huber_common::model::release::{Release, ReleaseIndex};
use huber_common::str::OsStrExt;
use is_executable::IsExecutable;
use log::{debug, info};
use maplit::hashmap;
use regex::{Captures, Regex};
use simpledi_rs::di::{DIContainer, DIContainerExtTrait, DependencyInjectTrait};
use symlink::{remove_symlink_dir, remove_symlink_file, symlink_dir, symlink_file};
use url::Url;
use urlencoding::decode;

use crate::file::trim_os_arch;
use crate::github::{GithubClient, GithubClientTrait};
use crate::service::package::PackageService;
use crate::service::{ItemOperationAsyncTrait, ItemOperationTrait, ItemSearchTrait, ServiceTrait};

const SUPPORTED_ARCHIVE_TYPES: [&str; 7] = ["tar.gz", "tar.xz", "zip", "gz", "xz", "tar", "tgz"];
const SUPPORTED_EXTRA_EXECUTABLE_TYPES: [&str; 4] = ["exe", "AppImage", "dmg", "msi"];

pub trait ReleaseTrait {
    fn current(&self, pkg: &Package) -> anyhow::Result<Release>;
    fn clean_current(&self, release: &Release) -> anyhow::Result<()>;
    fn reset_current(&self, pkg: &Package) -> anyhow::Result<()>;

    fn link_executables_for_current(
        &self,
        release: &Release,
        file: &Path,
    ) -> anyhow::Result<Option<Vec<String>>>;
    fn unlink_executables_for_current(&self, pkg: &Package, file: &Path) -> anyhow::Result<()>;

    fn get_executables_for_current(&self, pkg: &Package) -> anyhow::Result<Vec<String>>;
    fn delete_release(&self, release: &Release) -> anyhow::Result<()>;
}

#[async_trait]
pub trait ReleaseAsyncTrait {
    async fn download_install_github_package(
        &self,
        package: &Package,
        package_github: &GithubPackage,
    ) -> anyhow::Result<()>;

    async fn set_current(&self, release: &mut Release) -> anyhow::Result<Vec<String>>;
}

#[derive(Debug, Clone)]
pub struct ReleaseService {
    pub container: Option<Arc<DIContainer>>,
}

unsafe impl Send for ReleaseService {}

unsafe impl Sync for ReleaseService {}

impl ServiceTrait for ReleaseService {}

impl DependencyInjectTrait for ReleaseService {
    fn inject(&mut self, container: Arc<DIContainer>) {
        self.container = Some(container)
    }
}

impl Default for ReleaseService {
    fn default() -> Self {
        Self::new()
    }
}

impl ReleaseService {
    pub fn new() -> Self {
        Self { container: None }
    }

    pub async fn get_latest(&self, pkg: &Package) -> anyhow::Result<Release> {
        debug!("Getting the latest release: {}", pkg);

        let config = self.container.get::<Config>().unwrap();
        let client = GithubClient::new(config.to_github_credentials(), config.to_github_key_path());

        match &pkg.source {
            PackageSource::Github { owner, repo } => {
                client.get_latest_release(owner, repo, pkg).await
            }
        }
    }

    #[cfg(not(target_os = "windows"))]
    pub fn set_executable_permission(&self, path: &Path) -> anyhow::Result<()> {
        debug!("Making {:?} as executable", path);

        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o755))?;
        Ok(())
    }

    #[cfg(target_os = "windows")]
    pub fn set_executable_permission(&self, path: &PathBuf) -> anyhow::Result<()> {
        debug!("Ignored to make {:?} as executable", path);
        Ok(())
    }

    fn get_assets(package: &Package, version: &str) -> anyhow::Result<Vec<String>> {
        let asset_names: Vec<String> = package
            .target()?
            .artifact_templates
            .iter()
            .map(|it| {
                let regex = Regex::new(r"\{version:(\w)\}").unwrap();

                // v1.1.1 => v1_1_1 if the version separator is _, otherwise v1.1.1 instead
                if let Some(s) = regex.captures(it) {
                    let version_separator = s.get(1).unwrap().as_str();
                    regex
                        .replace(it, |_: &Captures| {
                            version
                                .trim_start_matches("v")
                                .replace(".", version_separator)
                        })
                        .to_string()
                } else {
                    it.replace("{version}", version.trim_start_matches("v"))
                        .to_string()
                }
            })
            .map(|it| {
                it.replace("{os}", env::consts::OS)
                    .replace("{arch}", env::consts::ARCH)
            })
            .collect();

        Ok(asset_names)
    }

    async fn download_assets(
        &self,
        package: &Package,
        config: &Config,
        version: &str,
        asset_download_urls: &mut Vec<String>,
    ) -> anyhow::Result<()> {
        let mut tasks = vec![];

        for download_url in asset_download_urls {
            // download
            info!("Downloading {}", &download_url);

            let pkg_dir = config.installed_pkg_dir(package, version)?;
            let filename = download_url.split("/").last().unwrap().to_string();
            let download_file_path = config.temp_dir()?.join(&filename);

            let task = async move {
                debug!("Saving {} to {:?}", &download_url, download_file_path);

                let _ = remove_file(&download_file_path);
                let _ = remove_dir_all(&download_file_path);

                let response = reqwest::get(download_url.to_string()).await?;
                match response.error_for_status() {
                    Err(e) => return Err(anyhow!("{:?}", e)),

                    Ok(response) => {
                        let mut dest_f = File::create(&download_file_path)?;
                        let bytes = response.bytes().await?;
                        dest_f.write_all(&bytes)?;
                    }
                }

                let ext = download_file_path.extension();
                if ext.is_none()
                    || !SUPPORTED_ARCHIVE_TYPES.contains(&ext.unwrap().to_str().unwrap())
                {
                    let dest_f = pkg_dir.join(&filename);

                    debug!(
                        "Moving {:?} to {:?}, because it's not an archive, regarded as an executable",
                        &download_file_path, &dest_f
                    );
                    self.set_executable_permission(&download_file_path)?;

                    let option = fs_extra::file::CopyOptions::new();
                    fs_extra::file::move_file(&download_file_path, &dest_f, &option)?;

                    return Ok(());
                }

                match ext {
                    None => debug!(
                        "Ignored {:?}, because it is not archived",
                        &download_file_path
                    ),

                    Some(ext) if ext.to_str().unwrap() == "exe" => {
                        debug!(
                            "Ignored {:?}, because it is not archived and with suffix 'exe'",
                            &download_file_path
                        );
                    }

                    Some(ext) => {
                        debug!("Decompressing {} which has extension {:?}", filename, ext);

                        let extract_dir = download_file_path.parent().unwrap().join("extract");
                        let download_file = File::open(&download_file_path)?;

                        debug!("Decompressing {:?} to {:?}", &download_file, &extract_dir);
                        fs::create_dir_all(&extract_dir)?;
                        uncompress_archive(&download_file, &extract_dir, Ownership::Ignore)?;

                        let dir = read_dir(&extract_dir)?;
                        let mut extract_content_dir = extract_dir.clone();
                        if dir.count() == 1 {
                            let dir = read_dir(&extract_dir)?;
                            let entry = dir.into_iter().next().unwrap()?;
                            extract_content_dir = entry.path();
                        }

                        let mut symbolic_links: HashMap<PathBuf, PathBuf> = hashmap! {};
                        let items_to_copy: Vec<PathBuf> = if extract_content_dir.is_dir() {
                            debug!("Moving {:?}/* to {:?}", &extract_content_dir, &pkg_dir);
                            extract_content_dir
                                .read_dir()?
                                .filter_map(|it| {
                                    let p = it.unwrap().path();

                                    match read_link(&p) {
                                        Ok(src_link) => {
                                            let dest_link = pkg_dir
                                                .join(p.file_name().unwrap().to_str_direct());
                                            symbolic_links.insert(dest_link, src_link.clone());

                                            None
                                        }

                                        Err(_) => Some(p),
                                    }
                                })
                                .collect()
                        } else {
                            debug!("Moving {:?} to {:?}", &extract_content_dir, &pkg_dir);
                            vec![extract_content_dir]
                        };

                        let mut option = fs_extra::dir::CopyOptions::new();
                        option.overwrite = true;
                        move_items(&items_to_copy, &pkg_dir, &option)?;

                        for (dest_link, src_link) in symbolic_links {
                            debug!("Add extra linked files {:?} to {:?}", src_link, dest_link);
                            symlink_file(src_link, dest_link)?
                        }

                        debug!(
                            "Removing temp files {:?}, {:?}",
                            download_file_path, extract_dir
                        );

                        let _ = remove_file(&download_file_path);
                        let _ = remove_dir_all(&download_file_path);
                        let _ = remove_dir_all(&extract_dir);
                    }
                };

                Ok(())
            };

            tasks.push(task);
        }

        futures::future::join_all(tasks)
            .await
            .iter()
            .all(|r| r.is_ok());
        Ok(())
    }
}

impl ReleaseTrait for ReleaseService {
    fn current(&self, pkg: &Package) -> anyhow::Result<Release> {
        debug!("Getting the current release: {}", &pkg);

        let config = self.container.get::<Config>().unwrap();
        let f = config.current_pkg_manifest_file(pkg)?;
        let f = File::open(f)?;

        // add linked executables in the release
        let mut release: Release = serde_yaml::from_reader(f)?;
        let executables = self.get_executables_for_current(&release.package)?;
        release.executables = Some(executables);

        Ok(release)
    }

    fn clean_current(&self, release: &Release) -> anyhow::Result<()> {
        debug!("Making {} not the current release", release);

        let config = self.container.get::<Config>().unwrap();

        let p = config.installed_pkg_manifest_file(&release.package, &release.version)?;
        let f = File::open(&p)?;
        let mut r: Release = serde_yaml::from_reader(&f)?;

        r.current = false;
        remove_file(&p)?;
        let f = File::create(&p)?;
        Ok(serde_yaml::to_writer(f, &r)?)
    }

    fn reset_current(&self, pkg: &Package) -> anyhow::Result<()> {
        debug!("Cleaning {} from the current manifests", &pkg);

        let config = self.container.get::<Config>().unwrap();

        // remove old symlink bin, current
        debug!("Removing the current package symbolic links: {}", &pkg);

        let current_pkg_dir = config.current_pkg_dir(pkg)?;
        let current_bin_dir = config.current_pkg_bin_dir(pkg)?;

        let mut scan_dirs: Vec<PathBuf> = pkg.get_scan_dirs(&current_pkg_dir)?;
        scan_dirs.push(current_pkg_dir.clone());
        scan_dirs.push(current_bin_dir.clone());

        for dir in scan_dirs {
            debug!("Scanning executables in {:?}", dir);

            if !dir.exists() {
                debug!("Ignored scanning {:?}, because it does not exist", dir);
                continue;
            }

            for entry in read_dir(&dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() {
                    self.unlink_executables_for_current(pkg, &path)?;
                }
            }
        }

        if current_pkg_dir.exists() {
            debug!("Removing link {:?}", &current_pkg_dir);
            remove_symlink_dir(&current_pkg_dir)?;
        }

        // remove it from index
        debug!("Removing {} from the current index", &pkg);

        let index_f = config.current_index_file()?;
        let indexes = if index_f.exists() {
            let f = File::open(&index_f)?;
            let indexes: Vec<ReleaseIndex> = serde_yaml::from_reader(&f)?;

            indexes
                .into_iter()
                .filter(|it| it.name != pkg.name)
                .collect()
        } else {
            vec![]
        };

        let _ = remove_file(&index_f);

        if !indexes.is_empty() {
            let index_f = File::create(&index_f)?;
            serde_yaml::to_writer(index_f, &indexes)?;
        }

        Ok(())
    }

    fn link_executables_for_current(
        &self,
        release: &Release,
        file: &Path,
    ) -> anyhow::Result<Option<Vec<String>>> {
        let config = self.container.get::<Config>().unwrap();
        let exec_filename = file.file_name().unwrap().to_str().unwrap().to_string();

        // if exec_templates specified, ignore not matched files
        let exec_templates: Vec<String> = release
            .package
            .target()?
            .executable_templates
            .unwrap_or(vec![]);
        if !exec_templates.is_empty() && !exec_templates.contains(&exec_filename) {
            debug!(
                "Ignored to link {:?} because it does not mentioned in executable_templates {:?}",
                &file, exec_templates
            );

            return Ok(None);
        }

        let mut exec_filenames = vec![exec_filename.clone()];

        // update linked exec name according to executable_mappings if it exists
        let exec_mappings: HashMap<String, String> = release
            .package
            .target()?
            .executable_mappings
            .unwrap_or(hashmap![]);
        if !exec_mappings.is_empty() {
            let re = Regex::new(r"(\d+.\d+.\d+)").unwrap();
            let expected_exec_filename = re.replace(&exec_filename, "{version}").to_string();

            if let Some(mapped_exec_names) = exec_mappings.get(&expected_exec_filename) {
                let re = Regex::new(r"\s+").unwrap();
                exec_filenames = re.split(mapped_exec_names).map(|x| x.to_string()).collect();
            }
        }

        let mut exec_file_paths = vec![];
        let mut find_exec_file_paths = |exec_filename: &str| -> anyhow::Result<_> {
            let exec_filename = trim_os_arch(exec_filename);
            let exec_file_path = config.bin_dir()?.join(&exec_filename);
            let _ = remove_file(&exec_file_path);

            if exec_filename.starts_with(".") {
                debug!(
                    "Ignored to link {:?} to {:?} because it's a hidden file",
                    &file, &exec_file_path
                );

                return Ok(());
            }

            // check if filename has invalid extension
            let exec_filename_without_version =
                exec_filename.as_str().replace(&release.version, "");
            let exec_file_path_without_version =
                file.parent().unwrap().join(&exec_filename_without_version);

            if let Some(ext) = exec_file_path_without_version.extension() {
                if !SUPPORTED_EXTRA_EXECUTABLE_TYPES.contains(&ext.to_str_direct()) {
                    debug!(
                        "Ignored to link {:?} to {:?} because of ext suffix {:?} not supported. {:?}",
                        &file, &exec_file_path, ext, SUPPORTED_EXTRA_EXECUTABLE_TYPES
                    );

                    return Ok(());
                }
            }

            if exec_filename_without_version
                .chars()
                .all(|x| x.is_uppercase())
                || exec_filename_without_version.starts_with("_")
                || exec_filename_without_version.starts_with(".")
            {
                debug!(
                    "Ignored to link {:?} to {:?} because of file name patterns (uppercase, class cass or starts with _)",
                    &file, &exec_file_path
                );

                return Ok(());
            }

            if file.extension().is_none() && !file.is_executable() {
                self.set_executable_permission(file)?;
            }

            if !file.is_executable() {
                debug!(
                    "Ignored to link {:?} to {:?} because it's not executable)",
                    &file, &exec_file_path
                );

                return Ok(());
            }

            debug!("Linking {:?} to {:?}", &file, &exec_file_path);
            symlink_file(file, &exec_file_path)?;
            exec_file_paths.push(exec_file_path.to_str().unwrap().to_string());

            Ok(())
        };

        for ref x in exec_filenames {
            find_exec_file_paths(x)?;
        }

        if exec_file_paths.is_empty() {
            Ok(None)
        } else {
            Ok(Some(exec_file_paths))
        }
    }

    fn unlink_executables_for_current(&self, pkg: &Package, file: &Path) -> anyhow::Result<()> {
        let config = self.container.get::<Config>().unwrap();
        let exec_filename = file.file_name().unwrap().to_str().unwrap().to_string();
        let mut exec_filenames = vec![exec_filename.clone()];

        // update linked exec name according to executable_mappings if it exists
        let exec_mappings: HashMap<String, String> =
            pkg.target()?.executable_mappings.unwrap_or(hashmap![]);
        if !exec_mappings.is_empty() {
            if let Some(mapped_exec_names) = exec_mappings.get(&exec_filename) {
                let re = Regex::new(r"\s+").unwrap();
                exec_filenames = re.split(mapped_exec_names).map(|x| x.to_string()).collect();
            }
        }

        for ref exec_filename in exec_filenames {
            let exec_filename = trim_os_arch(exec_filename);
            let exec_file_path = config.bin_dir()?.join(&exec_filename);

            if exec_file_path.exists() {
                debug!("Removing link {:?}", &exec_file_path);
                remove_symlink_file(exec_file_path)?;
            }
        }

        Ok(())
    }

    fn get_executables_for_current(&self, pkg: &Package) -> anyhow::Result<Vec<String>> {
        let config = self.container.get::<Config>().unwrap();
        let mut results: Vec<String> = vec![];

        let pkg_dir = config.current_pkg_dir(pkg)?;
        let pkg_bin_dir = config.current_pkg_bin_dir(pkg)?;
        let exec_mappings: HashMap<String, String> =
            pkg.target()?.executable_mappings.unwrap_or(hashmap![]);

        let scan_dirs = vec![pkg_dir, pkg_bin_dir];
        for dir in scan_dirs {
            debug!("Scanning executables in {:?}", dir);

            if !dir.exists() {
                debug!("Ignored scanning {:?}, because it does not exist", dir);
                continue;
            }

            for entry in read_dir(&dir)? {
                let entry = entry?;
                let path = entry.path();

                if path.is_file() {
                    let exec_filename = path.file_name().unwrap().to_str().unwrap().to_string();
                    let mut exec_filenames = vec![exec_filename.clone()];

                    if !exec_mappings.is_empty() {
                        if let Some(mapped_exec_names) = exec_mappings.get(&exec_filename) {
                            let re = Regex::new(r"\s+").unwrap();
                            exec_filenames =
                                re.split(mapped_exec_names).map(|x| x.to_string()).collect();
                        }
                    }

                    for ref exec_filename in exec_filenames {
                        let exec_filename = trim_os_arch(exec_filename);
                        let p = config.bin_dir()?.join(exec_filename);
                        if p.exists() {
                            results.push(p.to_str().unwrap().to_string());
                        }
                    }
                }
            }
        }

        Ok(results)
    }

    fn delete_release(&self, release: &Release) -> anyhow::Result<()> {
        debug!("Removing release: {}", &release);

        let cr = self.current(&release.package)?;
        if cr.version == release.version {
            return Err(anyhow!(
                "{} is the current release, unable to remove",
                release
            ));
        }

        let config = self.container.get::<Config>().unwrap();
        let p = config.installed_pkg_dir(&release.package, &release.version)?;

        Ok(remove_dir_all(p)?)
    }
}

#[async_trait]
impl ReleaseAsyncTrait for ReleaseService {
    async fn download_install_github_package(
        &self,
        package: &Package,
        package_github: &GithubPackage,
    ) -> anyhow::Result<()> {
        debug!("Downloading github package artifacts {}", &package);

        let config = self.container.get::<Config>().unwrap();
        let version = package.parse_version_from_tag_name(&package_github.tag_name)?;

        let mut asset_names = Self::get_assets(package, &version)?;
        let mut asset_download_urls: Vec<String> = vec![];
        let mut ext_asset_urls: Vec<String> = asset_names // external assets not on github
            .iter()
            .filter(|it| Url::parse(it).is_ok() && it.starts_with("https"))
            .cloned()
            .collect();
        asset_download_urls.append(&mut ext_asset_urls);

        if !ext_asset_urls.is_empty() {
            asset_names.retain(|it| !ext_asset_urls.contains(it));
        }

        // prepare download urls
        for asset in package_github.assets.iter() {
            let asset_download_url = decode(&asset.browser_download_url)?;

            // assets not mentioned in assert names, just ignored
            if !asset_names.contains(&asset.name)
                && !asset_names
                    .iter()
                    .any(|it| asset_download_url.ends_with(it))
            {
                debug!(
                    "Ignored {}, not mentioned or not right arch type defined in the package artifact config",
                    asset.name
                );
                continue;
            }

            asset_download_urls.push(asset_download_url.to_string());
        }

        if asset_download_urls.is_empty() {
            return Err(anyhow!(
                "No available artifacts for {} to download",
                package.name
            ));
        }

        // download
        self.download_assets(package, config, &version, &mut asset_download_urls)
            .await?;

        Ok(())
    }

    async fn set_current(&self, release: &mut Release) -> anyhow::Result<Vec<String>> {
        debug!("Setting the current release: {}", &release);

        let config = self.container.get::<Config>().unwrap();

        //TODO check locked version

        release.current = true;
        release.name = release.package.name.clone();

        let current_pkg_dir = config.current_pkg_dir(&release.package)?;
        let current_bin_dir = config.current_pkg_bin_dir(&release.package)?;

        let mut scan_dirs: Vec<PathBuf> = release.package.get_scan_dirs(&current_pkg_dir)?;
        scan_dirs.push(current_pkg_dir.clone());
        scan_dirs.push(current_bin_dir.clone());

        // remove old symlink bin, current
        debug!(
            "Removing the old current release symbolic links: {}",
            &release.package
        );
        self.reset_current(&release.package)?;

        // update current symlink
        debug!("Updating the current release symbolic links: {}", &release);

        let source: PathBuf = config.installed_pkg_dir(&release.package, &release.version)?;
        symlink_dir(&source, &current_pkg_dir)?;

        // scan executables in scan_dirs
        let mut linked_exe_files: Vec<String> = vec![];
        for dir in scan_dirs {
            debug!("Scanning executables in {:?}", dir);

            if !dir.exists() {
                debug!("Ignored scanning {:?}, because it does not exist", dir);
                continue;
            }

            for entry in read_dir(&dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() {
                    debug!("Scanning file: {:?}", path);
                    if let Some(mut paths) = self.link_executables_for_current(release, &path)? {
                        linked_exe_files.append(&mut paths);
                    }
                } else {
                    debug!("Ignored to scan non-file: {:?}", path);
                }
            }
        }

        let index_f = config.current_index_file()?;
        let mut indexes: Vec<ReleaseIndex> = vec![];

        // update old current release manifest
        debug!("Updating the current index manifest: {:?}", &index_f);

        if index_f.exists() {
            let f = File::open(&index_f)?;
            indexes = serde_yaml::from_reader(&f)?;

            if let Some(found) = indexes.iter().find(|it| it.name == release.package.name) {
                let old_pkg_manifest_path =
                    config.installed_pkg_manifest_file(&release.package, &found.version)?;
                let f = File::open(&old_pkg_manifest_path)?;

                let mut r: Release = serde_yaml::from_reader(f)?;
                r.current = false;

                let _ = remove_file(&old_pkg_manifest_path);
                let f = File::create(&old_pkg_manifest_path)?;
                serde_yaml::to_writer(f, &r)?;
            }

            indexes.retain(|it| it.name != release.package.name);
        }

        indexes.push(ReleaseIndex {
            name: release.package.name.clone(),
            version: release.version.clone(),
            owner: release.package.source.owner().to_string(),
            source: release.package.source.to_string(),
        });

        // update current release index file
        let _ = remove_file(&index_f);
        let index_f = File::create(&index_f)?;
        serde_yaml::to_writer(index_f, &indexes)?;

        let release_f = config.installed_pkg_manifest_file(&release.package, &release.version)?;
        let _ = remove_file(&release_f);
        let release_f = File::create(release_f)?;
        serde_yaml::to_writer(release_f, &release)?;

        // clean other installed releases as non-current
        let releases = self.find(&release.package).await?;
        let inactive_releases: Vec<&Release> = releases
            .iter()
            .filter(|it| it.version != release.version)
            .collect();

        for r in inactive_releases {
            self.clean_current(r)?;
        }

        Ok(linked_exe_files)
    }
}

impl ItemOperationTrait for ReleaseService {
    type Item = Package;
    type ItemInstance = Release;
    type Condition = Package;

    fn delete(&self, name: &str) -> anyhow::Result<()> {
        debug!("Deleting releases of package {}", name);

        let config = self.container.get::<Config>().unwrap();
        let pkg_service = self.container.get::<PackageService>().unwrap();
        let release_service = self.container.get::<ReleaseService>().unwrap();

        let pkg = pkg_service.get(name)?;
        let release = release_service.current(&pkg)?;

        self.reset_current(&release.package)?;

        let dir = config.installed_pkg_base_dir(&pkg)?;
        Ok(remove_dir_all(dir)?)
    }

    fn list(&self) -> anyhow::Result<Vec<Self::ItemInstance>> {
        debug!("Getting all current releases");

        let config = self.container.get::<Config>().unwrap();
        let mut releases: Vec<Release> = vec![];

        let index_f = config.current_index_file()?;
        if !index_f.exists() {
            return Ok(releases);
        }
        let index_f = File::open(index_f)?;

        let pkg_service = self.container.get::<PackageService>().unwrap();
        let indexes: Vec<ReleaseIndex> = serde_yaml::from_reader(index_f)?;

        for ri in indexes {
            match pkg_service.get(&ri.name) {
                Ok(pkg) => {
                    let p = config.installed_pkg_manifest_file(&pkg, &ri.version)?;

                    debug!("Reading {:?}", p);
                    match File::open(&p) {
                        Ok(f) => {
                            releases.push(serde_yaml::from_reader(f)?);
                        }
                        Err(e) => debug!(
                            "Failed to read {:?} and ignored from the installed release list: {}",
                            p, e
                        ),
                    }
                }
                Err(e) => {
                    return Err(anyhow!(
                        "Failed to get the installed {} package: {}",
                        &ri.name,
                        e
                    ));
                }
            }
        }

        Ok(releases)
    }

    fn get(&self, _name: &str) -> anyhow::Result<Self::ItemInstance> {
        unimplemented!()
    }
}

#[async_trait]
impl ItemOperationAsyncTrait for ReleaseService {
    type Item_ = Package;
    type ItemInstance_ = Release;
    type Condition_ = Package;

    async fn create(&self, obj: Self::Item_) -> anyhow::Result<Self::ItemInstance_> {
        debug!("Creating release from package: {}", &obj);

        if self.has(&obj.name)? {
            return Err(anyhow!("{} already installed", &obj.name));
        }

        self.update(&obj).await
    }

    async fn update(&self, obj: &Self::Item_) -> anyhow::Result<Self::ItemInstance_> {
        debug!("Updating release from package: {}", &obj);

        let config = self.container.get::<Config>().unwrap();
        let client = GithubClient::new(config.to_github_credentials(), config.to_github_key_path());

        // Get the release from GitHub
        let mut release = match &obj.source {
            PackageSource::Github { owner, repo } => match &obj.version {
                Some(v) => {
                    debug!("Getting {} of package release {}", &v, &obj);
                    client.get_release(owner, repo, v, obj).await?
                }
                None => {
                    debug!("Getting the latest release of package {}", &obj);

                    if let Ok(r) = client.get_latest_release(owner, repo, obj).await {
                        r
                    } else {
                        debug!("Getting the latest pre-release of package {}", &obj);
                        client
                            .get_releases(owner, repo, obj)
                            .await?
                            .first()
                            .expect("")
                            .to_owned()
                    }
                }
            },
        };

        let release_detail = release.package.detail.as_ref();
        if release_detail.is_none() {
            return Err(anyhow!("No matched release detail found: {}", release));
        }

        match release_detail.unwrap() {
            PackageDetailType::Github { package: p } => {
                debug!(
                    "Downloading package artifacts from github {:?}",
                    obj.source.url()
                );
                self.download_install_github_package(obj, p).await?;

                debug!("Setting {} as the current package", release);
                let executables = self.set_current(&mut release).await?;

                info!("Installed executables:\n{:#?}", &executables);
                release.executables = Some(executables);

                Ok(release)
            }
        }
    }

    async fn find(&self, pkg: &Self::Condition_) -> anyhow::Result<Vec<Self::ItemInstance_>> {
        debug!("Finding releases by condition: {}", &pkg);

        let config = self.container.get::<Config>().unwrap();

        let mut releases: Vec<Release> = vec![];

        let pkg_base_dir = config.installed_pkg_base_dir(pkg)?;
        for entry in read_dir(&pkg_base_dir)? {
            let entry = entry?;
            let filename = entry.file_name();
            let filename = filename.to_str().unwrap();

            if filename == "current" {
                continue;
            }

            if entry.path().is_dir() {
                let p = config.installed_pkg_manifest_file(pkg, filename)?;
                let f = File::open(p)?;
                let r: Release = serde_yaml::from_reader(f)?;

                releases.push(r);
            }
        }

        Ok(releases)
    }
}

impl ItemSearchTrait for ReleaseService {
    type SearchItem = Release;

    fn search(
        &self,
        name: Option<&str>,
        _pattern: Option<&str>,
        _owner: Option<&str>,
    ) -> anyhow::Result<Vec<Self::SearchItem>> {
        debug!("Searching releases");

        let mut found_items: Vec<Self::SearchItem> = vec![];
        let releases = self.list()?;

        for r in releases.iter() {
            if name.is_some() && r.package.name != name.unwrap() {
                continue;
            }

            let mut updated_r = r.clone();

            if releases.iter().any(|it| it.version == r.version) {
                updated_r.current = true;
            }

            found_items.push(updated_r);
        }

        Ok(found_items)
    }
}

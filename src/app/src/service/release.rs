use std::collections::HashMap;
use std::fs;
use std::fs::{read_dir, read_link, remove_dir_all, remove_file, File};
use std::io::Write;
use std::path::PathBuf;
use std::sync::Arc;

use async_trait::async_trait;
use compress_tools::{uncompress_archive, Ownership};
use fs_extra::move_items;
use inflector::cases::uppercase::is_upper_case;
use is_executable::IsExecutable;
use log::{debug, info};
use semver::Version;
use simpledi_rs::di::{DIContainer, DIContainerExtTrait, DependencyInjectTrait};
use symlink::{remove_symlink_dir, remove_symlink_file, symlink_dir, symlink_file};
use url::Url;
use urlencoding::decode;

use huber_common::file::trim_os_arch;
use huber_common::model::config::{Config, ConfigFieldConvertTrait, ConfigPath};
use huber_common::model::package::{GithubPackage, Package, PackageDetailType, PackageSource};
use huber_common::model::release::{Release, ReleaseIndex};
use huber_common::progress::{ProgressBar, ProgressTrait};
use huber_common::result::Result;
use huber_common::str::OsStrExt;

use crate::component::github::{GithubClient, GithubClientTrait};
use crate::service::package::PackageService;
use crate::service::{ItemOperationAsyncTrait, ItemOperationTrait, ItemSearchTrait, ServiceTrait};

const SUPPORTED_ARCHIVE_TYPES: [&str; 7] = ["tar.gz", "tar.xz", "zip", "gz", "xz", "tar", "tgz"];
const SUPPORTED_EXTRA_EXECUTABLE_TYPES: [&str; 3] = ["exe", "AppImage", "dmg"];

pub(crate) trait ReleaseTrait {
    fn current(&self, pkg: &Package) -> Result<Release>;
    fn clean_current(&self, release: &Release) -> Result<()>;
    fn reset_current(&self, pkg: &Package) -> Result<()>;

    fn link_executables_for_current(
        &self,
        release: &Release,
        file: &PathBuf,
    ) -> Result<Option<String>>;
    fn unlink_executables_for_current(&self, pkg: &Package, file: &PathBuf) -> Result<()>;

    fn get_executables_for_current(&self, pkg: &Package) -> Result<Vec<String>>;
    fn delete_release(&self, release: &Release) -> Result<()>;
}

#[async_trait]
pub(crate) trait ReleaseAsyncTrait {
    async fn download_install_github_package(
        &self,
        package: &Package,
        package_github: &GithubPackage,
    ) -> Result<()>;

    async fn set_current(&self, release: &mut Release) -> Result<Vec<String>>;
}

#[derive(Debug)]
pub(crate) struct ReleaseService {
    pub(crate) container: Option<Arc<DIContainer>>,
}

unsafe impl Send for ReleaseService {}

unsafe impl Sync for ReleaseService {}

impl ServiceTrait for ReleaseService {}

impl DependencyInjectTrait for ReleaseService {
    fn inject(&mut self, container: Arc<DIContainer>) {
        self.container = Some(container)
    }
}

impl ReleaseService {
    pub(crate) fn new() -> Self {
        Self { container: None }
    }

    pub(crate) async fn get_latest(&self, pkg: &Package) -> Result<Release> {
        debug!("Getting the latest release: {}", pkg);

        let config = self.container.get::<Config>().unwrap();
        let client = GithubClient::new(config.to_github_credentials(), config.to_github_key_path());

        match &pkg.source {
            PackageSource::Github { owner, repo } => {
                client.get_latest_release(&owner, &repo, pkg).await
            }

            _ => unimplemented!(),
        }
    }

    #[cfg(not(target_os = "windows"))]
    pub(crate) fn set_executable_permission(&self, path: &PathBuf) -> Result<()> {
        info!("Making {:?} as executable", path);

        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o755))?;
        Ok(())
    }

    #[cfg(target_os = "windows")]
    pub(crate) fn set_executable_permission(&self, path: &PathBuf) -> Result<()> {
        info!("Ignored to make {:?} as executable", path);
        Ok(())
    }
}

impl ReleaseTrait for ReleaseService {
    fn current(&self, pkg: &Package) -> Result<Release> {
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

    fn clean_current(&self, release: &Release) -> Result<()> {
        debug!("Making {} not the current release", release);

        let config = self.container.get::<Config>().unwrap();

        let p = config.installed_pkg_manifest_file(&release.package, &release.version)?;
        let f = File::open(&p)?;
        let mut r: Release = serde_yaml::from_reader(&f)?;

        r.current = false;
        let _ = remove_file(&p)?;
        let f = File::create(&p)?;
        Ok(serde_yaml::to_writer(f, &r)?)
    }

    fn reset_current(&self, pkg: &Package) -> Result<()> {
        info!("Cleaning {} from the current manifests", &pkg);

        let config = self.container.get::<Config>().unwrap();

        // remove old symlink bin, current
        info!("Removing the current package symbolic links: {}", &pkg);

        let current_pkg_dir = config.current_pkg_dir(&pkg)?;
        let current_bin_dir = config.current_pkg_bin_dir(&pkg)?;

        let mut scan_dirs: Vec<PathBuf> = pkg.get_scan_dirs(&current_pkg_dir)?;
        scan_dirs.push(current_pkg_dir.clone());
        scan_dirs.push(current_bin_dir.clone());

        for dir in scan_dirs {
            info!("Scanning executables in {:?}", dir);

            if !dir.exists() {
                info!("Ignored scanning {:?}, because it does not exist", dir);
                continue;
            }

            for entry in read_dir(&dir)?.into_iter() {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() {
                    self.unlink_executables_for_current(&pkg, &path)?;
                }
            }
        }

        if current_pkg_dir.exists() {
            info!("Removing link {:?}", &current_pkg_dir);
            remove_symlink_dir(&current_pkg_dir)?;
        }

        // remove it from index
        info!("Removing {} from the current index", &pkg);

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
        file: &PathBuf,
    ) -> Result<Option<String>> {
        let config = self.container.get::<Config>().unwrap();
        let mut exec_filename = file.file_name().unwrap().to_str().unwrap().to_string();

        // if exec_templates specified, ignore not matched files
        let exec_templates: Vec<String> = release
            .package
            .target()?
            .executable_templates
            .unwrap_or(vec![]);
        if exec_templates.len() > 0 && !exec_templates.contains(&exec_filename) {
            info!(
                "Ignored to link {:?} because it does not mentioned in executable_templates {:?}",
                &file, exec_templates
            );

            return Ok(None);
        }

        // update linked exec name according to executable_mappings if it exists
        let exec_mappings: HashMap<String, String> = release
            .package
            .target()?
            .executable_mappings
            .unwrap_or(hashmap![]);
        if exec_mappings.len() > 0 {
            let regex = regex::Regex::new(r"(\d+.\d+.\d+)").unwrap();
            let expected_exec_filename = regex.replace(&exec_filename, "{version}").to_string();

            if let Some(mapped_exec_name) = exec_mappings.get(&expected_exec_filename) {
                exec_filename = mapped_exec_name.clone();
            }
        }

        let exec_filename = trim_os_arch(&exec_filename);
        let exec_file_path = config.bin_dir()?.join(&exec_filename);

        if exec_file_path.exists() {
            let _ = remove_file(&exec_file_path);
        }

        if exec_filename.starts_with(".") {
            info!(
                "Ignored to link {:?} to {:?} because it's a hidden file",
                &file, &exec_file_path
            );

            return Ok(None);
        }

        // check if filename has invalid extension
        let exec_filename_without_version = exec_filename.as_str().replace(&release.version, "");
        let exec_file_path_without_version =
            file.parent().unwrap().join(&exec_filename_without_version);

        if let Some(ext) = exec_file_path_without_version.extension() {
            if !SUPPORTED_EXTRA_EXECUTABLE_TYPES.contains(&ext.to_str_direct()) {
                info!(
                    "Ignored to link {:?} to {:?} because of suffix {:?}",
                    &file, &exec_file_path, ext
                );

                return Ok(None);
            }
        }

        if is_upper_case(exec_filename_without_version.clone())
            || exec_filename_without_version.starts_with("_")
            || exec_filename_without_version.starts_with(".")
        {
            info!(
                "Ignored to link {:?} to {:?} because of file name patterns (uppercase, class cass or starts with _)",
                &file, &exec_file_path
            );

            return Ok(None);
        }

        if file.extension().is_none() && !file.is_executable() {
            self.set_executable_permission(file)?;
        }

        if !file.is_executable() {
            info!(
                "Ignored to link {:?} to {:?} because it's not executable)",
                &file, &exec_file_path
            );

            return Ok(None);
        }

        info!("Linking {:?} to {:?}", &file, &exec_file_path);
        symlink_file(file, &exec_file_path)?;

        Ok(Some(exec_file_path.to_str().unwrap().to_string()))
    }

    fn unlink_executables_for_current(&self, pkg: &Package, file: &PathBuf) -> Result<()> {
        let config = self.container.get::<Config>().unwrap();
        let mut exec_filename = file.file_name().unwrap().to_str().unwrap().to_string();

        // update linked exec name according to executable_mappings if it exists
        let exec_mappings: HashMap<String, String> =
            pkg.target()?.executable_mappings.unwrap_or(hashmap![]);
        if exec_mappings.len() > 0 {
            if let Some(mapped_exec_name) = exec_mappings.get(&exec_filename) {
                exec_filename = mapped_exec_name.clone();
            }
        }

        let exec_filename = trim_os_arch(&exec_filename);
        let exec_file_path = config.bin_dir()?.join(&exec_filename);

        if exec_file_path.exists() {
            info!("Removing link {:?}", &exec_file_path);
            remove_symlink_file(exec_file_path)?;
        }

        Ok(())
    }

    fn get_executables_for_current(&self, pkg: &Package) -> Result<Vec<String>> {
        let config = self.container.get::<Config>().unwrap();
        let mut results: Vec<String> = vec![];

        let pkg_dir = config.current_pkg_dir(&pkg)?;
        let pkg_bin_dir = config.current_pkg_bin_dir(&pkg)?;
        let exec_mappings: HashMap<String, String> =
            pkg.target()?.executable_mappings.unwrap_or(hashmap![]);

        let scan_dirs = vec![pkg_dir, pkg_bin_dir];
        for dir in scan_dirs {
            debug!("Scanning executables in {:?}", dir);

            if !dir.exists() {
                debug!("Ignored scanning {:?}, because it does not exist", dir);
                continue;
            }

            for entry in read_dir(&dir)?.into_iter() {
                let entry = entry?;
                let path = entry.path();

                if path.is_file() {
                    let mut exec_filename = path.file_name().unwrap().to_str().unwrap().to_string();

                    if exec_mappings.len() > 0 {
                        if let Some(mapped_exec_name) = exec_mappings.get(&exec_filename) {
                            exec_filename = mapped_exec_name.clone();
                        }
                    }

                    let exec_filename = trim_os_arch(&exec_filename);
                    let p = config.bin_dir()?.join(exec_filename);
                    if p.exists() {
                        results.push(p.to_str().unwrap().to_string());
                    }
                }
            }
        }

        Ok(results)
    }

    fn delete_release(&self, release: &Release) -> Result<()> {
        info!("Removing release: {}", &release);

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
    ) -> Result<()> {
        info!("Downloading github package artifacts {}", &package);

        let config = self.container.get::<Config>().unwrap();
        let version = package.parse_version_from_tag_name(&package_github.tag_name)?;

        let mut asset_names: Vec<String> = package
            .target()?
            .artifact_templates
            .iter()
            .map(|it| it.replace("{version}", &version.trim_start_matches("v")))
            .collect();

        let mut download_urls: Vec<String> = vec![];
        let mut asset_urls: Vec<String> = asset_names
            .iter()
            .filter(|it| Url::parse(it).is_ok() && it.starts_with("https"))
            .map(|it| it.clone())
            .collect();
        download_urls.append(&mut asset_urls);

        if !asset_urls.is_empty() {
            asset_names = asset_names
                .into_iter()
                .filter(|it| !asset_urls.contains(it))
                .collect();
        }

        // prepare download urls
        for a in package_github.assets.iter() {
            let decoded_download_url = decode(&a.browser_download_url)?;

            if !asset_names.contains(&a.name)
                && !asset_names
                    .iter()
                    .any(|it| decoded_download_url.ends_with(it))
            {
                continue;
            }

            download_urls.push(decoded_download_url);
        }

        // download
        let mut tasks = vec![];

        for download_url in download_urls {
            // download
            info!("Downloading {}", &download_url);

            let pkg_dir = config.installed_pkg_dir(package, &version)?;
            let filename = download_url.split("/").last().unwrap().to_string();
            let download_file_path = config.temp_dir()?.join(&filename);

            let task = async move {
                info!("Saving {} to {:?}", &download_url, download_file_path);

                let _ = remove_file(&download_file_path);
                let _ = remove_dir_all(&download_file_path);

                let response = reqwest::get(&download_url).await?;
                match response.error_for_status() {
                    Err(e) => return Err(anyhow!("{:?}", e)),

                    Ok(response) => {
                        let mut dest_f = File::create(&download_file_path)?;
                        let bytes = response.bytes().await?;
                        dest_f.write(&bytes)?;
                    }
                }

                let ext = download_file_path.extension();
                if ext.is_none()
                    || !SUPPORTED_ARCHIVE_TYPES.contains(&ext.unwrap().to_str().unwrap())
                {
                    let dest_f = pkg_dir.join(&filename);

                    info!(
                        "Moving {:?} to {:?}, because it's not an archive, regarded as an executable",
                        &download_file_path, &dest_f
                    );
                    self.set_executable_permission(&download_file_path)?;

                    let option = fs_extra::file::CopyOptions::new();
                    fs_extra::file::move_file(&download_file_path, &dest_f, &option)?;

                    return Ok(());
                }

                match ext {
                    None => info!(
                        "Ignored {:?}, because it is not archived",
                        &download_file_path
                    ),

                    Some(ext) if ext.to_str().unwrap() == "exe" => {
                        info!(
                            "Ignored {:?}, because it is not archived and w/ suffix 'exe'",
                            &download_file_path
                        );
                    }

                    Some(ext) => {
                        // uncompress
                        info!("Decompressing {} which has extension {:?}", filename, ext);

                        let extract_dir = download_file_path.join("extract");
                        let download_file = File::open(&download_file_path)?;

                        info!("Decompressing {:?} to {:?}", &download_file, &extract_dir);
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
                            info!("Moving {:?}/* to {:?}", &extract_content_dir, &pkg_dir);
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
                            info!("Moving {:?} to {:?}", &extract_content_dir, &pkg_dir);
                            vec![extract_content_dir]
                        };

                        let mut option = fs_extra::dir::CopyOptions::new();
                        option.overwrite = true;
                        move_items(&items_to_copy, &pkg_dir, &option)?;

                        for (dest_link, src_link) in symbolic_links {
                            info!("Add extra linked files {:?} to {:?}", src_link, dest_link);
                            symlink_file(src_link, dest_link)?
                        }

                        info!(
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

        futures::future::join_all(tasks).await;
        Ok(())
    }

    async fn set_current(&self, release: &mut Release) -> Result<Vec<String>> {
        info!("Setting the current release: {}", &release);

        let config = self.container.get::<Config>().unwrap();
        release.current = true;
        release.name = release.package.name.clone();

        let current_pkg_dir = config.current_pkg_dir(&release.package)?;
        let current_bin_dir = config.current_pkg_bin_dir(&release.package)?;

        let mut scan_dirs: Vec<PathBuf> = release.package.get_scan_dirs(&current_pkg_dir)?;
        scan_dirs.push(current_pkg_dir.clone());
        scan_dirs.push(current_bin_dir.clone());

        // remove old symlink bin, current
        info!(
            "Removing the current release symbolic links: {}",
            &release.package
        );

        self.reset_current(&release.package)?;

        // update current symlink
        info!("Updating the current release symbolic links: {}", &release);

        let source: PathBuf = config.installed_pkg_dir(&release.package, &release.version)?;
        symlink_dir(&source, &current_pkg_dir)?;

        // scan executables in scan_dirs
        let mut linked_exe_files: Vec<String> = vec![];
        for dir in scan_dirs {
            info!("Scanning executables in {:?}", dir);

            if !dir.exists() {
                info!("Ignored scanning {:?}, because it does not exist", dir);
                continue;
            }

            for entry in read_dir(&dir)?.into_iter() {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() {
                    if let Some(p) = self.link_executables_for_current(&release, &path)? {
                        linked_exe_files.push(p);
                    }
                }
            }
        }

        let index_f = config.current_index_file()?;
        let mut indexes: Vec<ReleaseIndex> = vec![];

        // update old current release manifest
        info!("Updating the current index manifest: {:?}", &index_f);

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

            indexes = indexes
                .into_iter()
                .filter(|it| it.name != release.package.name)
                .collect();
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
            self.clean_current(&r)?;
        }

        Ok(linked_exe_files)
    }
}

impl ItemOperationTrait for ReleaseService {
    type Item = Package;
    type ItemInstance = Release;
    type Condition = Package;

    fn delete(&self, name: &str) -> Result<()> {
        info!("Deleting releases of package {}", name);

        let config = self.container.get::<Config>().unwrap();
        let pkg_service = self.container.get::<PackageService>().unwrap();

        let pkg = pkg_service.get(name)?;
        self.reset_current(&pkg)?;

        let dir = config.installed_pkg_base_dir(&pkg)?;
        Ok(remove_dir_all(dir)?)
    }

    fn list(&self) -> Result<Vec<Self::ItemInstance>> {
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
            let pkg = pkg_service.get(&ri.name)?;
            let p = config.installed_pkg_manifest_file(&pkg, &ri.version)?;

            let f = File::open(p)?;
            releases.push(serde_yaml::from_reader(f)?);
        }

        Ok(releases)
    }

    fn get(&self, _name: &str) -> Result<Self::ItemInstance> {
        unimplemented!()
    }
}

#[async_trait]
impl ItemOperationAsyncTrait for ReleaseService {
    type Item_ = Package;
    type ItemInstance_ = Release;
    type Condition_ = Package;

    async fn create(&self, obj: Self::Item_) -> Result<Self::ItemInstance_> {
        info!("Creating release from package: {}", &obj);

        if self.has(&obj.name)? {
            return Err(anyhow!("{} already installed", &obj.name));
        }

        self.update(&obj).await
    }

    async fn update(&self, obj: &Self::Item_) -> Result<Self::ItemInstance_> {
        info!("Updating release from package: {}", &obj);

        let config = self.container.get::<Config>().unwrap();
        let client = GithubClient::new(config.to_github_credentials(), config.to_github_key_path());

        // get the release from github
        let mut release = match &obj.source {
            PackageSource::Github { owner, repo } => match &obj.version {
                Some(v) => client.get_release(&owner, &repo, &v, &obj).await?,
                None => client.get_latest_release(&owner, &repo, &obj).await?,
            },

            _ => unimplemented!(),
        };

        let release_detail = release.package.detail.as_ref();
        if release_detail.is_none() {
            return Err(anyhow!("No matched release detail found: {}", release));
        }

        match release_detail.unwrap() {
            PackageDetailType::Github { package: p } => {
                progress!(
                    format!(
                        "Downloading package artifacts from github {:?}",
                        obj.source.url()
                    ),
                    self.download_install_github_package(&obj, &p).await?;
                );

                let executables = progress!(
                    format!("Setting {} as the current package", release),
                    self.set_current(&mut release).await?;
                );

                println!(
                    "{}",
                    format!("Installed executables:\n - {}", executables.join("\n - "))
                        .trim_end_matches("- ")
                );
                release.executables = Some(executables);

                Ok(release)
            }
        }
    }

    async fn find(&self, pkg: &Self::Condition_) -> Result<Vec<Self::ItemInstance_>> {
        debug!("Finding releases by condition: {}", &pkg);

        let config = self.container.get::<Config>().unwrap();

        let mut releases: Vec<Release> = vec![];

        let pkg_base_dir = config.installed_pkg_base_dir(&pkg)?;
        for entry in read_dir(&pkg_base_dir)?.into_iter() {
            let entry = entry?;
            let filename = entry.file_name();
            let filename = filename.to_str().unwrap();

            if entry.path().is_dir() {
                if let Ok(_) = Version::parse(filename.trim_start_matches("v")) {
                    let p = config.installed_pkg_manifest_file(&pkg, &filename)?;
                    let f = File::open(p)?;
                    let r: Release = serde_yaml::from_reader(f)?;

                    releases.push(r);
                }
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
    ) -> Result<Vec<Self::SearchItem>> {
        debug!("Searching releases");

        let mut found_items: Vec<Self::SearchItem> = vec![];
        let releases = self.list()?;

        for r in releases.iter() {
            if name.is_some() && r.package.name != name.unwrap() {
                continue;
            }

            let mut updated_r = r.clone();

            if releases.iter().find(|it| it.version == r.version).is_some() {
                updated_r.current = true;
            }

            found_items.push(updated_r);
        }

        Ok(found_items)
    }
}

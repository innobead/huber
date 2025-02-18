use std::collections::HashMap;
use std::fs::{read_dir, read_link, remove_dir_all, remove_file, File};
use std::io::Write;
use std::path::PathBuf;
use std::sync::Arc;
use std::{env, fs};

use anyhow::anyhow;
use async_trait::async_trait;
use filepath::FilePath;
use fs_extra::move_items;
use is_executable::IsExecutable;
use log::{debug, error, info};
use maplit::hashmap;
use regex::Regex;
use simpledi_rs::di::{DIContainer, DIContainerExtTrait, DependencyInjectTrait};
use symlink::{remove_symlink_dir, remove_symlink_file, symlink_dir, symlink_file};
use url::Url;
use urlencoding::decode;

use crate::cmd::PlatformStdLib;
use crate::compress::uncompress_archive;
use crate::fs::has_suffix;
use crate::fs::set_executable_permission;
use crate::gh::{GithubClient, GithubClientTrait};
use crate::model::config::{Config, ConfigFieldConvertTrait, ConfigPath};
use crate::model::package::{GithubPackage, Package, PackageDetailType, PackageSource};
use crate::model::release::{Release, ReleaseIndex};
use crate::os::{is_os_arch_match, trim_os_arch_version};
use crate::service::package::PackageService;
use crate::service::{ItemOperationAsyncTrait, ItemOperationTrait, ItemSearchTrait, ServiceTrait};

const SUPPORTED_ARCHIVE_TYPES: [&str; 6] = ["tar.gz", "tar.xz", "zip", "tar", "tgz", "gz"];

pub trait ReleaseTrait {
    fn current(&self, pkg: &Package) -> anyhow::Result<Release>;
    fn clean_current(&self, release: &Release) -> anyhow::Result<()>;
    fn reset_current(&self, pkg: &Package) -> anyhow::Result<()>;

    fn unlink_executables_for_current(&self, pkg: &Package) -> anyhow::Result<()>;

    fn get_executables_for_current(
        &self,
        pkg: &Package,
        symlink: bool,
    ) -> anyhow::Result<Vec<String>>;
    fn delete_release(&self, release: &Release) -> anyhow::Result<()>;
}

#[async_trait]
pub trait ReleaseAsyncTrait {
    async fn download_install_github_package(
        &self,
        package: &Package,
        package_github: &GithubPackage,
        prefer_stdlib: &PlatformStdLib,
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

    pub async fn update(
        &self,
        obj: &Package,
        prefer_stdlib: &PlatformStdLib,
        release_check: bool,
    ) -> anyhow::Result<Release> {
        debug!("Updating release from package: {:#?}", &obj);

        let config = self.container.get::<Config>().unwrap();
        let client = GithubClient::new(config.to_github_credentials(), config.to_github_key_path());

        // Get the release from GitHub
        let mut release = if release_check {
            match obj.source {
                PackageSource::Github {
                    ref owner,
                    ref repo,
                } => match obj.version {
                    Some(ref v) => {
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
                                .expect("Failed to find the first release")
                                .to_owned()
                        }
                    }
                },
            }
        } else {
            Release {
                package: obj.clone(),
                version: obj.version.clone().unwrap_or_default(),
                current: false,
                executables: None,
                name: "".to_string(),
                kind: None,
            }
        };

        let release_detail = release.package.detail.clone().or_else(|| {
            Some(PackageDetailType::Github {
                package: GithubPackage {
                    tag_name: obj.version.clone().unwrap_or_default(),
                    ..Default::default()
                },
            })
        });
        if release_detail.is_none() {
            return Err(anyhow!("No matched release detail found: {}", release));
        }

        match release_detail.unwrap() {
            PackageDetailType::Github { package: p } => {
                debug!(
                    "Downloading package artifacts from github {:?}",
                    obj.source.url()
                );
                self.download_install_github_package(obj, &p, prefer_stdlib)
                    .await?;

                debug!("Setting {} as the current package", release);
                let executables = self.set_current(&mut release).await?;
                info!(
                    "Installed executables of {}:\n{:#?}",
                    obj.name, &executables
                );
                release.executables = Some(executables);

                Ok(release)
            }
        }
    }

    fn get_assets(package: &Package, version: &str) -> anyhow::Result<Vec<String>> {
        let asset_names: Vec<String> = package
            .target()?
            .artifact_templates
            .iter()
            .map(|it| {
                it.replace("{version}", version.trim_start_matches("v"))
                    .replace("{os}", env::consts::OS)
                    .replace("{arch}", env::consts::ARCH)
            })
            .filter(|it| {
                let file_name = if let Ok(url) = Url::parse(it) {
                    url.path_segments().unwrap().last().unwrap().to_string()
                } else {
                    it.clone()
                };

                if file_name.contains('.') && !file_name.ends_with(".exe") {
                    SUPPORTED_ARCHIVE_TYPES
                        .iter()
                        .any(|ext| file_name.ends_with(ext))
                } else {
                    true
                }
            })
            .collect();

        Ok(asset_names)
    }

    async fn download_assets(
        &self,
        package: &Package,
        config: &Config,
        version: &str,
        download_urls: &mut Vec<String>,
    ) -> anyhow::Result<()> {
        let mut tasks = vec![];

        'download: for download_url in download_urls {
            let pkg_dir = config.installed_pkg_dir(package, version)?;
            let filename = download_url.split("/").last().unwrap().to_string();
            let download_file_path = config.temp_dir()?.join(&filename);

            let mut ext = "";
            let trimmed_filename = trim_os_arch_version(&filename);

            for (i, &archive_type) in SUPPORTED_ARCHIVE_TYPES.iter().enumerate() {
                if trimmed_filename.ends_with(archive_type) {
                    ext = archive_type;
                    break;
                }
                if !has_suffix(&trimmed_filename) {
                    break;
                }
                if i == SUPPORTED_ARCHIVE_TYPES.len() - 1 {
                    debug!("Ignored to download {} due to unsupported archive type. Supported types: {:?}",&download_url, SUPPORTED_ARCHIVE_TYPES);
                    continue 'download;
                }
            }

            let task = async move {
                info!("Downloading {}", &download_url);
                debug!("Downloading {} to {:?}", &download_url, &download_file_path);

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

                // downloaded asset seems an executable instead of an archive, move it to the package directory
                if ext.is_empty() {
                    let dest_f = pkg_dir.join(&filename);

                    debug!(
                        "Moving {:?} to {:?}, because it's not an archive, regarded as an executable",
                        &download_file_path, &dest_f
                    );
                    set_executable_permission(&download_file_path)?;

                    let option = fs_extra::file::CopyOptions {
                        overwrite: false,
                        skip_exist: true,
                        buffer_size: 0,
                    };
                    fs_extra::file::move_file(&download_file_path, &dest_f, &option)?;

                    return Ok(());
                }

                Self::decompress_asset(&pkg_dir, &filename, &download_file_path, ext)?;

                Ok(())
            };

            tasks.push(task);
        }

        let downloaded = futures::future::join_all(tasks)
            .await
            .iter()
            .map(|r| {
                if let Err(e) = r {
                    error!("Failed to download asset: {}", e);
                }
                r.as_ref()
            })
            .all(|r| r.is_ok());

        if !downloaded {
            return Err(anyhow!("Failed to download assets"));
        }

        Ok(())
    }

    fn decompress_asset(
        pkg_dir: &PathBuf,
        filename: &str,
        download_file_path: &PathBuf,
        ext: &str,
    ) -> anyhow::Result<()> {
        debug!("Decompressing {} which has extension {:?}", filename, ext);

        let extract_dir = download_file_path.parent().unwrap().join("extract");
        let download_file = File::open(download_file_path)?;

        debug!("Decompressing {:?} to {:?}", &download_file, &extract_dir);

        fs::create_dir_all(&extract_dir)?;
        uncompress_archive(&download_file.path()?, &extract_dir, ext)?;

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
                            let dest_link = pkg_dir.join(p.file_name().unwrap().to_str().unwrap());
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
        move_items(&items_to_copy, pkg_dir, &option)?;

        for (dest_link, src_link) in symbolic_links {
            debug!("Add extra linked files {:?} to {:?}", src_link, dest_link);
            symlink_file(src_link, dest_link)?
        }

        debug!(
            "Removing temp files {:?}, {:?}",
            download_file_path, extract_dir
        );

        let _ = remove_file(download_file_path);
        let _ = remove_dir_all(download_file_path);
        let _ = remove_dir_all(&extract_dir);
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
        let executables = self.get_executables_for_current(&release.package, false)?;
        release.executables = Some(executables);

        Ok(release)
    }

    fn clean_current(&self, release: &Release) -> anyhow::Result<()> {
        debug!("Cleaning the current release: {}", &release);

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
        if current_pkg_dir.exists() {
            self.unlink_executables_for_current(pkg)?;

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

    fn unlink_executables_for_current(&self, pkg: &Package) -> anyhow::Result<()> {
        let exec_paths = self.get_executables_for_current(pkg, false)?;
        for ref exec_path in exec_paths {
            debug!("Removing link {:?}", exec_path);
            remove_symlink_file(exec_path)?;
        }

        Ok(())
    }

    //noinspection ALL
    fn get_executables_for_current(
        &self,
        pkg: &Package,
        symlink: bool,
    ) -> anyhow::Result<Vec<String>> {
        let config = self.container.get::<Config>().unwrap();
        let mut results: Vec<String> = vec![];

        let pkg_dir = config.current_pkg_dir(pkg)?;
        let pkg_bin_dir = config.current_pkg_bin_dir(pkg)?;
        let exec_mappings: HashMap<_, _> = pkg.target()?.executable_mappings.unwrap_or_default();

        let semver_regex = Regex::new(
            r"v?(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)
(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+([0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?",
        )?;

        for dir in &[pkg_dir, pkg_bin_dir] {
            debug!("Scanning executables in {:?}", dir);

            if !dir.exists() {
                debug!("Ignored scanning {:?}, because it does not exist", dir);
                continue;
            }

            for entry in read_dir(dir)? {
                let exec_path = entry?.path();
                if !exec_path.is_executable() {
                    debug!("Ignored non-executable {:?}", exec_path);
                    continue;
                } else if has_suffix(&trim_os_arch_version(
                    &exec_path.file_name().unwrap().to_string_lossy(),
                )) {
                    debug!("Ignored executable {:?} due to suffix", exec_path);
                    continue;
                }

                let mut exec_name = exec_path.file_name().unwrap().to_string_lossy().to_string();
                exec_name = semver_regex.replace(&exec_name, "").to_string();
                exec_name = exec_mappings
                    .get(&exec_name)
                    .unwrap_or(&exec_name)
                    .to_string();

                let exec_link = config.bin_dir()?.join(trim_os_arch_version(&exec_name));
                if symlink {
                    let _ = remove_file(&exec_link);
                    symlink_file(&exec_path, &exec_link)?;
                }
                if exec_link.exists() {
                    results.push(exec_link.to_string_lossy().to_string());
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
        prefer_stdlib: &PlatformStdLib,
    ) -> anyhow::Result<()> {
        debug!("Downloading github package artifacts {}", &package);

        let config = self.container.get::<Config>().unwrap();
        let version = package.parse_version_from_tag_name(&package_github.tag_name)?;

        let mut asset_names = Self::get_assets(package, &version)?;

        let mut ext_asset_urls: Vec<String> = asset_names // external assets not on github
            .iter()
            .filter(|it| Url::parse(it).is_ok() && it.starts_with("https"))
            .cloned()
            .collect();
        let mut asset_download_urls: Vec<String> = vec![];
        asset_download_urls.append(&mut ext_asset_urls);

        if !ext_asset_urls.is_empty() {
            asset_names.retain(|it| !ext_asset_urls.contains(it));
        }

        for asset in package_github.assets.iter() {
            let asset_url = decode(&asset.browser_download_url)?.to_string();

            if asset_names.is_empty() {
                debug!(
                    "Checking {} if it's in the expected os/arch type: {}, since there are no expected artifact names defined",
                    asset.name, asset_url
                );
                if !is_os_arch_match(
                    env::consts::OS,
                    env::consts::ARCH,
                    &asset_url.to_lowercase(),
                ) {
                    continue;
                }
            } else {
                debug!(
                    "Checking {} if it's in the expected artifact names: {:?}",
                    asset.name, asset_names
                );
                if !asset_names.contains(&asset.name)
                    && !asset_names.iter().any(|it| asset_url.ends_with(it))
                {
                    debug!(
                    "Ignored {}, not mentioned or not right arch type defined in the package artifact config",
                    asset.name
                );
                    continue;
                }
            }

            debug!("Found asset: {}", asset_url);
            asset_download_urls.push(asset_url.to_string());
        }

        if !package_github.assets.is_empty() && asset_download_urls.is_empty() {
            return Err(anyhow!(
                "No available artifacts for {} to download. Expected artifact names: {:?}",
                package.name,
                asset_names
            ));
        }

        if prefer_stdlib != &PlatformStdLib::None {
            info!(
                "Prefer downloading assets for {} belonging to the specified stdlib: {}",
                package.name,
                prefer_stdlib
            );

            let stdlib_regex = Regex::new(&format!(
                r"\b{}\b?",
                prefer_stdlib.to_string().to_lowercase()
            ))?;
            let results: Vec<_> = asset_download_urls
                .clone()
                .into_iter()
                .filter(|it| {
                    let filename = Url::parse(it)
                        .unwrap()
                        .path_segments()
                        .and_then(|segments| segments.last())
                        .unwrap_or_default()
                        .to_string();
                    stdlib_regex.is_match(&filename)
                })
                .collect();
            if !results.is_empty() {
                asset_download_urls = results;
            }
        }

        self.download_assets(package, config, &version, &mut asset_download_urls)
            .await?;

        Ok(())
    }

    async fn set_current(&self, release: &mut Release) -> anyhow::Result<Vec<String>> {
        debug!("Setting the current release: {}", &release);

        release.current = true;
        release.name = release.package.name.clone();

        debug!(
            "Removing the old current release symbolic links: {}",
            &release.package
        );
        self.reset_current(&release.package)?;

        debug!("Updating the current release symbolic links: {}", &release);

        let config = self.container.get::<Config>().unwrap();
        let current_pkg_dir = config.current_pkg_dir(&release.package)?;
        let source = config.installed_pkg_dir(&release.package, &release.version)?;

        symlink_dir(&source, &current_pkg_dir)?;

        let linked_exe_files = self.get_executables_for_current(&release.package, true)?;
        if linked_exe_files.is_empty() {
            return Err(anyhow!("No executables found when installing {}", &release));
        }

        // update old current release manifest
        let index_f = config.current_index_file()?;
        let mut indexes: Vec<ReleaseIndex> = vec![];

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

        self.update(&obj, &PlatformStdLib::None, true).await
    }

    async fn update(&self, _obj: &Self::Item_) -> anyhow::Result<Self::ItemInstance_> {
        unimplemented!()
    }

    async fn find(&self, pkg: &Self::Condition_) -> anyhow::Result<Vec<Self::ItemInstance_>> {
        debug!("Finding releases by condition: {}", &pkg);

        let config = self.container.get::<Config>().unwrap();

        let mut releases: Vec<Release> = vec![];

        let pkg_base_dir = config.installed_pkg_base_dir(pkg)?;
        for entry in read_dir(&pkg_base_dir)? {
            let entry = entry?;
            let filename = entry.file_name().to_string_lossy().to_string();

            if filename == "current" {
                continue;
            }

            if entry.path().is_dir() {
                let p = config.installed_pkg_manifest_file(pkg, &filename)?;
                if !p.exists() {
                    debug!("Ignored {:?}, because the manifest file does not exist", p);
                    continue;
                }

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
        _repo: Option<&str>,
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

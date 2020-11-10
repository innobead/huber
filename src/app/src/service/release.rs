use std::fs;
use std::fs::{read_dir, remove_dir_all, remove_file, File};
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::sync::Arc;

use compress_tools::{uncompress_archive, Ownership};
use fs_extra::move_items;
use inflector::cases::classcase::is_class_case;
use inflector::cases::uppercase::is_upper_case;
use is_executable::IsExecutable;
use log::{debug, info};
use semver::Version;
use symlink::{remove_symlink_dir, remove_symlink_file, symlink_dir, symlink_file};
use tokio::runtime::Runtime;
use url::Url;
use urlencoding::decode;

use huber_common::config::Config;
use huber_common::di::di_container;
use huber_common::file::trim_os_arch;
use huber_common::model::package::{GithubPackage, Package, PackageDetailType, PackageSource};
use huber_common::model::release::{Release, ReleaseIndex};
use huber_common::result::Result;

use crate::component::github::{GithubClient, GithubClientTrait};
use crate::service::package::PackageService;
use crate::service::{ItemOperationTrait, ItemSearchTrait};
use std::collections::HashMap;

pub(crate) trait ReleaseTrait {
    fn current(&self, pkg: &Package) -> Result<Release>;
    fn set_current(&self, release: &mut Release) -> Result<Vec<String>>;
    fn link_executables_for_current(&self, release: &Release, file: &PathBuf) -> Result<Option<String>>;
    fn unlink_executables_for_current(&self, pkg: &Package, file: &PathBuf) -> Result<()>;
    fn get_executables_for_current(&self, pkg: &Package) -> Result<Vec<String>>;
    fn delete_release(&self, release: &Release) -> Result<()>;
    fn download_install_github_package(
        &self,
        package: &Package,
        package_github: &GithubPackage,
    ) -> Result<()>;
    fn clean_current(&self, pkg: &Package) -> Result<()>;
}

#[derive(Debug)]
pub(crate) struct ReleaseService {
    pub(crate) config: Option<Arc<Config>>,
    pub(crate) runtime: Option<Arc<Runtime>>,
}

impl ReleaseService {
    pub(crate) fn new() -> Self {
        Self {
            config: None,
            runtime: None,
        }
    }

    pub(crate) fn get_latest(&self, pkg: &Package) -> Result<Release> {
        debug!("Getting the latest release: {}", &pkg);

        let config = self.config.as_ref().unwrap();

        let client = GithubClient::new(
            config.github_credentials.clone(),
            config.git_ssh_key.clone(),
        );
        let mut runtime = Runtime::new().unwrap();

        runtime.block_on(async {
            match &pkg.source {
                PackageSource::Github { owner, repo } => {
                    client.get_latest_release(&owner, &repo, &pkg).await
                }
                _ => unimplemented!(),
            }
        })
    }
}

impl ReleaseTrait for ReleaseService {
    fn current(&self, pkg: &Package) -> Result<Release> {
        debug!("Getting the current release: {}", &pkg);

        let f = self
            .config
            .as_ref()
            .unwrap()
            .current_pkg_manifest_file(pkg)?;
        let f = File::open(f)?;

        // add linked executables in the release
        let mut release: Release = serde_yaml::from_reader(f)?;
        let executables = self.get_executables_for_current(&release.package)?;
        release.executables = Some(executables);

        Ok(release)
    }

    fn set_current(&self, release: &mut Release) -> Result<Vec<String>> {
        info!("Setting the current release: {}", &release);

        let config = self.config.as_ref().unwrap();
        release.current = true;
        release.name = release.package.name.clone();

        let current_pkg_dir = config.current_pkg_dir(&release.package)?;
        let current_bin_dir = config.current_pkg_bin_dir(&release.package)?;

        // remove old symlink bin, current
        info!(
            "Removing the current release symbolic links: {}",
            &release.package
        );
        self.clean_current(&release.package)?;

        // update current symlink
        info!("Updating the current release symbolic links: {}", &release);
        let source: PathBuf = config.installed_pkg_dir(&release.package, &release.version)?;
        symlink_dir(&source, &current_pkg_dir)?;

        info!(
            "Updating the current release bin symbolic links: {}",
            &release
        );
        let mut linked_exe_files: Vec<String> = vec![];
        let scan_dirs = vec![&current_pkg_dir, &current_bin_dir];
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

        Ok(linked_exe_files)
    }

    fn link_executables_for_current(&self, release: &Release, file: &PathBuf) -> Result<Option<String>> {
        let config = self.config.as_ref().unwrap();
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

            return Ok(None)
        }

        // update linked exec name according to executable_mappings if it exists
        let exec_mappings: HashMap<String, String> = release
            .package
            .target()?
            .executable_mappings
            .unwrap_or(hashmap![]);
        if exec_mappings.len() > 0 {
            if let Some(mapped_exec_name) = exec_mappings.get(&exec_filename) {
                exec_filename = mapped_exec_name.clone();
            }
        }

        let exec_filename = trim_os_arch(&exec_filename);
        let exec_file_path = config.bin_dir()?.join(&exec_filename);

        // check if filename has invalid extension
        let exec_filename_without_version = exec_filename.as_str().replace(&release.version, "");
        let exec_file_path_without_version =
            file.parent().unwrap().join(&exec_filename_without_version);

        if let Some(ext) = exec_file_path_without_version.extension() {
            info!(
                "Ignored to link {:?} to {:?} because of suffix {:?}",
                &file, &exec_file_path, ext
            );

            return Ok(None)
        }

        if is_upper_case(exec_filename_without_version.clone())
            || is_class_case(exec_filename_without_version.clone())
            || exec_filename_without_version.starts_with("_")
        {
            info!(
                "Ignored to link {:?} to {:?} because of file name patterns (uppercase, class cass or starts with _)",
                &file, &exec_file_path
            );

            return Ok(None);
        }

        if file.extension().is_none() && !file.is_executable() {
            info!("Making {:?} as executable", &file);
            fs::set_permissions(&file, fs::Permissions::from_mode(0o755))?;
        }

        if !file.is_executable() {
            info!(
                "Ignored to link {:?} to {:?} because it's not executable)",
                &file, &exec_file_path
            );

            return Ok(None)
        }

        info!("Linking {:?} to {:?}", &file, &exec_file_path);
        symlink_file(file, &exec_file_path)?;

        Ok(Some(exec_file_path.to_str().unwrap().to_string()))
    }

    fn unlink_executables_for_current(&self, pkg: &Package, file: &PathBuf) -> Result<()> {
        let config = self.config.as_ref().unwrap();
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
        let config = self.config.as_ref().unwrap();
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

        let config = self.config.as_ref().unwrap();
        let p = config.installed_pkg_dir(&release.package, &release.version)?;
        Ok(remove_dir_all(p)?)
    }

    fn download_install_github_package(
        &self,
        package: &Package,
        package_github: &GithubPackage,
    ) -> Result<()> {
        info!("Downloading github package artifacts {}", &package);

        let config = self.config.as_ref().unwrap();
        let supported_archive_types = vec!["tar.gz", "tar.xz", "zip", "gz", "xz", "tar", "tgz"];

        let version = &package_github.tag_name;
        let pkg_mgmt = package.target()?;
        let mut asset_names: Vec<String> = pkg_mgmt
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

        // let runtime = self.runtime.as_ref().unwrap();
        let mut runtime = Runtime::new().unwrap();
        runtime.block_on(async {
            //TODO need to check checksume
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

            for download_url in download_urls {
                // download
                info!("Downloading {}", &download_url);

                let response = reqwest::get(&download_url).await?;
                let pkg_dir = config.installed_pkg_dir(package, &version)?;
                let filename = download_url.split("/").last().unwrap();
                let download_file_path = config.temp_dir()?.join(filename);

                info!("Saving {} to {:?}", &download_url, download_file_path);

                let _ = remove_file(&download_file_path);
                let _ = remove_dir_all(&download_file_path);

                let mut dest_f = File::create(&download_file_path)?;
                let bytes = response.bytes().await?;
                dest_f.write(&bytes)?;

                let ext = download_file_path.extension();
                debug!("{:?}", ext);
                if ext.is_none()
                    || !supported_archive_types.contains(&ext.unwrap().to_str().unwrap())
                {
                    let dest_f = pkg_dir.join(&filename);

                    info!("Moving {:?} to {:?}, because it's not an archive, regarded as an executable", &download_file_path, &dest_f);

                    fs::set_permissions(&download_file_path, fs::Permissions::from_mode(0o755)).unwrap();
                    let option = fs_extra::file::CopyOptions::new();
                    fs_extra::file::move_file(
                        &download_file_path,
                        &dest_f,
                        &option,
                    )?;

                    continue;
                }

                match ext {
                    None => info!("Ignored {:?}, because it is not executable and archived", &download_file_path),

                    Some(ext) => {
                        // uncompress
                        info!("Decompressing {} which has extension {:?}", filename, ext);

                        let extract_dir = download_file_path.join("extract");
                        let download_f = File::open(&download_file_path)?;

                        info!("Decompressing {:?} to {:?}", &download_f, &extract_dir);
                        uncompress_archive(&download_f, &extract_dir, Ownership::Ignore)?;

                        let dir = read_dir(&extract_dir)?;
                        let mut extra_content_dir = extract_dir.clone();
                        if dir.count() == 1 {
                            let dir = read_dir(&extract_dir)?;
                            let entry = dir.into_iter().next().unwrap()?;
                            extra_content_dir = entry.path();
                        }

                        let copy_items: Vec<PathBuf> = if extra_content_dir.is_dir() {
                            info!("Moving {:?}/* to {:?}", &extra_content_dir, &pkg_dir);

                            extra_content_dir.read_dir()?.map(|it| {
                                it.unwrap().path()
                            }).collect()
                        } else {
                            info!("Moving {:?} to {:?}", &extra_content_dir, &pkg_dir);
                            vec![extra_content_dir]
                        };

                        let mut option = fs_extra::dir::CopyOptions::new();
                        option.overwrite = true;
                        move_items(&copy_items, &pkg_dir, &option)?;

                        info!("Removing temp files {:?}, {:?}", download_file_path, extract_dir);

                        let _ = remove_file(&download_file_path);
                        let _ = remove_dir_all(&download_file_path);
                        let _ = remove_dir_all(&extract_dir);
                    }
                }
            }

            Ok(())
        })
    }

    fn clean_current(&self, pkg: &Package) -> Result<()> {
        info!("Cleaning {} from the current manifests", &pkg);

        let config = self.config.as_ref().unwrap();

        let current_dir = config.current_pkg_dir(&pkg)?;
        let current_bin_dir = config.current_pkg_bin_dir(&pkg)?;

        // remove old symlink bin, current
        info!("Removing the current package symbolic links: {}", &pkg);
        let scan_dirs = vec![&current_dir, &current_bin_dir];
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

        if current_dir.exists() {
            info!("Removing link {:?}", &current_dir);
            remove_symlink_dir(&current_dir)?;
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
}

impl ItemOperationTrait for ReleaseService {
    type Item = Package;
    type ItemInstance = Release;
    type Condition = Package;

    fn create(&self, obj: &Self::Item) -> Result<Self::ItemInstance> {
        info!("Creating release from package: {}", &obj);

        if self.has(&obj.name)? {
            return Err(anyhow!("{} already installed", &obj.name));
        }

        self.update(&obj)
    }

    fn update(&self, obj: &Self::Item) -> Result<Self::ItemInstance> {
        info!("Updating release from package: {}", &obj);

        let config = self.config.as_ref().unwrap();
        let client = GithubClient::new(
            config.github_credentials.clone(),
            config.git_ssh_key.clone(),
        );

        // get the release from github
        //FIXME let runtime = self.runtime.as_ref().unwrap();
        let mut runtime = Runtime::new().unwrap();
        let mut release = runtime.block_on(async {
            match &obj.source {
                PackageSource::Github { owner, repo } => match &obj.version {
                    Some(v) => client.get_release(&owner, &repo, &v, &obj).await,
                    None => client.get_latest_release(&owner, &repo, &obj).await,
                },

                _ => unimplemented!(),
            }
        })?;

        let release_detail = release.package.detail.as_ref();
        if release_detail.is_none() {
            return Err(anyhow!("No matched release detail found: {}", release));
        }

        match release_detail.unwrap() {
            PackageDetailType::Github { package: p } => {
                println!("Downloading package artifacts from github");
                self.download_install_github_package(obj, &p)?;

                println!("Setting {} as the current package", release);
                let executables = self.set_current(&mut release)?;

                println!("{}", format!("Installed executables:\n - {}", executables.join("\n - ")).trim_end_matches("- "));
                release.executables = Some(executables);

                Ok(release)
            }
        }
    }

    fn delete(&self, name: &str) -> Result<()> {
        info!("Deleting releases of package {}", name);

        let config = self.config.as_ref().unwrap();
        let container = di_container();
        let pkg_service = container.get::<PackageService>().unwrap();

        let pkg = pkg_service.get(name)?;
        self.clean_current(&pkg)?;

        let dir = config.installed_pkg_base_dir(&pkg)?;
        Ok(remove_dir_all(dir)?)
    }

    fn list(&self) -> Result<Vec<Self::ItemInstance>> {
        debug!("Getting all current releases");

        let config = self.config.as_ref().unwrap();
        let container = di_container();
        let mut releases: Vec<Release> = vec![];

        let index_f = config.current_index_file()?;
        if !index_f.exists() {
            return Ok(releases);
        }
        let index_f = File::open(index_f)?;

        let pkg_service = container.get::<PackageService>().unwrap();
        let indexes: Vec<ReleaseIndex> = serde_yaml::from_reader(index_f)?;

        for ri in indexes {
            let pkg = pkg_service.get(&ri.name)?;
            let p = config.installed_pkg_manifest_file(&pkg, &ri.version)?;

            let f = File::open(p)?;
            releases.push(serde_yaml::from_reader(f)?);
        }

        Ok(releases)
    }

    fn find(&self, pkg: &Self::Condition) -> Result<Vec<Self::ItemInstance>> {
        debug!("Finding releases by condition: {}", &pkg);

        let config = self.config.as_ref().unwrap();

        let mut releases: Vec<Release> = vec![];
        let current_pkg = self.current(&pkg)?;

        let pkg_base_dir = config.installed_pkg_base_dir(&pkg)?;
        for entry in read_dir(&pkg_base_dir)?.into_iter() {
            let entry = entry?;
            let filename = entry.file_name();
            let filename = filename.to_str().unwrap();

            if entry.path().is_dir() {
                if let Ok(_) = Version::parse(filename.trim_start_matches("v")) {
                    releases.push(Release {
                        name: pkg.name.clone(),
                        version: filename.to_string(),
                        current: current_pkg.version == filename.to_string(),
                        package: pkg.clone(),
                        executables: None,
                    });
                }
            }
        }

        Ok(releases)
    }

    fn get(&self, _name: &str) -> Result<Self::ItemInstance> {
        unimplemented!()
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

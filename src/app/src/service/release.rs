use std::fs;
use std::fs::{copy, read_dir, remove_dir_all, remove_file, File};
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::sync::Arc;

use compress_tools::{uncompress_archive, Ownership};
use inflector::cases::classcase::is_class_case;
use inflector::cases::uppercase::is_upper_case;
use is_executable::IsExecutable;
use log::{debug, info};
use semver::Version;
use symlink::{remove_symlink_dir, remove_symlink_file, symlink_dir, symlink_file};
use tempdir::TempDir;
use tokio::runtime::Runtime;
use url::Url;
use urlencoding::decode;
use walkdir::WalkDir;

use huber_common::config::Config;
use huber_common::di::di_container;
use huber_common::model::package::{GithubPackage, Package, PackageDetailType, PackageSource};
use huber_common::model::release::{Release, ReleaseIndex};
use huber_common::result::Result;

use crate::component::github::{GithubClient, GithubClientTrait};
use crate::service::package::PackageService;
use crate::service::{ItemOperationTrait, ItemSearchTrait};

pub(crate) trait ReleaseTrait {
    fn current(&self, pkg: &Package) -> Result<Release>;
    fn set_current(&self, release: &mut Release) -> Result<()>;
    fn delete_release(&self, release: &Release) -> Result<()>;
    fn download_install_github_package(
        &self,
        package: &Package,
        package_github: &GithubPackage,
    ) -> Result<Vec<String>>;
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

        Ok(serde_yaml::from_reader(f)?)
    }

    fn set_current(&self, release: &mut Release) -> Result<()> {
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
        symlink_dir(source, current_pkg_dir)?;

        info!(
            "Updating the current release bin symbolic links: {}",
            &release
        );
        for entry in read_dir(&current_bin_dir)?.into_iter() {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                let filename = path.file_name().unwrap().to_os_string();
                let exec_path = config.bin_dir()?.join(&filename);

                // check if filename has invalid extension
                let filename = filename.to_str().unwrap().replace(&release.version, "");
                let p = config.bin_dir()?.join(&filename);

                if p.extension().is_some() {
                    if let Some(ext) = path.extension() {
                        info!(
                            "Ignored to link {:?} to {:?} because of suffix {:?}",
                            &path, &exec_path, ext
                        );
                        continue;
                    }
                }

                info!("Linking {:?} to {:?}", &path, &exec_path);
                symlink_file(path, exec_path)?;
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

        Ok(())
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
    ) -> Result<Vec<String>> {
        info!("Downloading github package artifacts {}", &package);

        let config = self.config.as_ref().unwrap();
        let supported_archive_types = vec!["tar.gz", "zip", "gz", "tar"];

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
            let mut file_paths: Vec<String> = vec![];

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
                let filename = download_url.split("/").last().unwrap();

                let dest_root_path = config.installed_pkg_bin_dir(package, &version)?;
                let dest_path = dest_root_path.join(filename);
                let mut dest_f = File::create(&dest_path)?;
                let bytes = response.bytes().await?;
                dest_f.write(&bytes)?;

                if filename.ends_with(".sh") {
                    fs::set_permissions(&dest_path, fs::Permissions::from_mode(0o755)).unwrap();

                    file_paths.push(dest_path.to_str().unwrap().to_string());
                } else {
                    let ext = dest_path.extension();
                    if ext.is_none()
                        || !supported_archive_types.contains(&ext.unwrap().to_str().unwrap())
                    {
                        fs::set_permissions(&dest_path, fs::Permissions::from_mode(0o755)).unwrap();

                        file_paths.push(dest_path.to_str().unwrap().to_string());
                        continue;
                    }

                    if let Some(ext) = ext {
                        // uncompress
                        info!("Decompressing {} which has extension {:?}", filename, ext);
                        let extract_dir = TempDir::new(filename)?;
                        let dest_f = File::open(&dest_path)?;

                        info!("Decompressing {:?} to {:?}", &dest_f, extract_dir.path());
                        uncompress_archive(&dest_f, extract_dir.path(), Ownership::Ignore)?;
                        let _ = remove_file(&dest_path);

                        // copy executables to bin
                        let walker = WalkDir::new(&extract_dir).into_iter();
                        for entry in
                            walker.filter(|it| it.as_ref().unwrap().metadata().unwrap().is_file())
                        {
                            let entry = entry?;
                            let f = entry.path();

                            // uncompress, copy executables to bin folder
                            let filename = f.file_name().unwrap().to_str().unwrap().to_string();
                            if is_upper_case(filename.clone())
                                || is_class_case(filename.clone())
                                || filename.starts_with("_")
                            {
                                debug!("Ignored {:?}", &filename);
                                continue;
                            }

                            if f.is_executable() || f.extension().is_none() {
                                let dest_f = dest_root_path.join(f.file_name().unwrap());

                                info!("Moving executables {:?} to {:?}", &f, &dest_f);
                                copy(&f, &dest_f)?;

                                if f.extension().is_none() {
                                    info!("Making {:?} as executable", &dest_f);
                                    fs::set_permissions(
                                        &dest_f,
                                        fs::Permissions::from_mode(0o755),
                                    )?;
                                }

                                file_paths.push(dest_f.to_str().unwrap().to_string())
                            } else {
                                debug!("Ignored {:?}", &f);
                            }
                        }

                        info!("Removing temp dir {:?}", extract_dir.path());
                        let _ = remove_dir_all(extract_dir);
                    }
                }
            }

            if file_paths.is_empty() {
                return Err(anyhow!("No valid artifacts found"));
            }

            Ok(file_paths)
        })
    }

    fn clean_current(&self, pkg: &Package) -> Result<()> {
        info!("Cleaning {} from the current manifests", &pkg);

        let config = self.config.as_ref().unwrap();

        let pkg_dir = config.current_pkg_dir(&pkg)?;
        let pkg_bin_dir = config.current_pkg_bin_dir(&pkg)?;

        // remove old symlink bin, current
        if pkg_bin_dir.exists() {
            info!("Removing old symbolic links: {}", &pkg);

            for entry in read_dir(&pkg_bin_dir)?.into_iter() {
                let entry = entry?;
                let path = entry.path();

                if path.is_file() {
                    let exec_path = config
                        .bin_dir()?
                        .join(path.file_name().unwrap().to_os_string());

                    if exec_path.exists() {
                        info!("Removing link {:?}", &exec_path);
                        remove_symlink_file(exec_path)?;
                    }
                }
            }
        }

        if pkg_dir.exists() {
            info!("Removing link {:?}", &pkg_dir);
            remove_symlink_dir(&pkg_dir)?;
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
                self.set_current(&mut release)?;

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

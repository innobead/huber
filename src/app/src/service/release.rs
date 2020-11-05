use std::env::temp_dir;
use std::fs::{copy, read_dir, remove_dir_all, remove_file, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use compress_tools::{uncompress_archive, Ownership};
use faccess::PathExt;
use symlink::{remove_symlink_dir, symlink_dir};
use tokio::runtime::Runtime;
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
    fn set_current(&self, release: &Release) -> Result<()>;
    fn delete_release(&self, release: &Release) -> Result<()>;
    fn download_install_github_package(
        &self,
        package: &Package,
        package_github: &GithubPackage,
    ) -> Result<Vec<File>>;
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
}

impl ReleaseTrait for ReleaseService {
    fn current(&self, pkg: &Package) -> Result<Release> {
        let f = self
            .config
            .as_ref()
            .unwrap()
            .current_pkg_manifest_file(pkg)?;
        let f = File::open(f)?;

        Ok(serde_yaml::from_reader(f)?)
    }

    fn set_current(&self, release: &Release) -> Result<()> {
        let config = self.config.as_ref().unwrap();

        let f = config.current_pkg_dir(&release.package)?;
        if f.exists() {
            remove_symlink_dir(&f)?;
        }

        let source: PathBuf = config.installed_pkg_dir(&release.package, &release.version)?;
        Ok(symlink_dir(source, f)?)
    }

    fn delete_release(&self, release: &Release) -> Result<()> {
        let current_r = self.current(&release.package)?;

        if current_r.version == release.version {
            return Err(anyhow!(
                "{} is the current release, not able to delete",
                release
            ));
        }

        let config = self.config.as_ref().unwrap();
        config
            .installed_pkg_dir(&release.package, &release.version)
            .map(|_| ())
    }

    fn download_install_github_package(
        &self,
        package: &Package,
        package_github: &GithubPackage,
    ) -> Result<Vec<File>> {
        let config = self.config.as_ref().unwrap();

        let version = &package_github.tag_name;
        let pkg_mgmt = package.target()?;
        let asset_names: Vec<String> = pkg_mgmt
            .artifact_templates
            .iter()
            .map(|it| it.replace("{version}", &version))
            .collect();

        // let runtime = self.runtime.as_ref().unwrap();
        let mut runtime = Runtime::new().unwrap();
        runtime.block_on(async {
            let mut files: Vec<File> = vec![];

            for a in package_github.assets.iter() {
                if !asset_names.contains(&a.name) {
                    continue;
                }

                // download
                let response = reqwest::get(&a.browser_download_url).await?;
                let filename = response
                    .url()
                    .path_segments()
                    .and_then(|segments| segments.last())
                    .expect("The downloaded file not found");

                let dest_root_path = config.installed_pkg_bin_dir(package, &version)?;
                let dest_path = dest_root_path.join(filename);
                let mut dest_f = File::create(&dest_path)?;
                let bytes = response.bytes().await?;
                dest_f.write(&bytes)?;

                match dest_path.extension() {
                    None => files.push(dest_f),

                    Some(_) => {
                        // unarchive
                        let extract_dir = temp_dir();
                        uncompress_archive(&dest_f, &extract_dir, Ownership::Preserve)?;
                        let _ = remove_file(&dest_path);

                        // find executables, move to bin
                        let walker = WalkDir::new(&extract_dir).into_iter();
                        for entry in walker.filter_entry(|it| it.metadata().unwrap().is_file()) {
                            let entry = entry?;
                            let f = Path::new(entry.file_name());

                            // move to bin
                            if f.executable() {
                                let dest_f = dest_root_path.join(f.file_name().unwrap());
                                copy(&f, dest_f)?;
                            }
                        }

                        let _ = remove_dir_all(extract_dir);
                    }
                }
            }

            Ok(files)
        })
    }
}

impl ItemOperationTrait for ReleaseService {
    type Item = Package;
    type ItemInstance = Release;
    type Condition = String;

    fn create(&self, obj: &Self::Item) -> Result<Self::ItemInstance> {
        if self.has(&obj.name)? {
            return Err(anyhow!("{} already installed", &obj.name));
        }

        self.update(&obj)
    }

    fn update(&self, obj: &Self::Item) -> Result<Self::ItemInstance> {
        let config = self.config.as_ref().unwrap();
        let client = GithubClient::new(
            config.github_credentials.clone(),
            config.git_ssh_key.clone(),
        );

        // get the release from github
        //FIXME let runtime = self.runtime.as_ref().unwrap();
        let mut runtime = Runtime::new().unwrap();
        let release = runtime.block_on(async {
            match &obj.source {
                PackageSource::Github { owner, repo } => match &obj.version {
                    Some(v) => client.get_release(&owner, &repo, &v).await,
                    None => client.get_latest_release(&owner, &repo).await,
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
                self.download_install_github_package(obj, &p)?;
                self.set_current(&release)?;

                Ok(release)
            }
        }
    }

    fn delete(&self, name: &str) -> Result<()> {
        let config = self.config.as_ref().unwrap();
        let container = di_container();
        let pkg_service = container.get::<PackageService>().unwrap();

        let pkg = pkg_service.get(name)?;
        let dir = config.installed_pkg_base_dir(&pkg)?;

        for f in read_dir(&dir)? {
            let pkg_version_dir = f?.path();
            let _ = remove_dir_all(pkg_version_dir);
        }

        Ok(())
    }

    fn list(&self) -> Result<Vec<Self::ItemInstance>> {
        let config = self.config.as_ref().unwrap();
        let index_f = config.current_index_file()?;
        let f = File::open(index_f)?;

        let container = di_container();
        let pkg_service = container.get::<PackageService>().unwrap();
        let mut releases: Vec<Release> = vec![];

        let indexes: Vec<ReleaseIndex> = serde_yaml::from_reader(f)?;
        for ri in indexes {
            let pkg = pkg_service.get(&ri.name)?;
            let p = config.installed_pkg_manifest_file(&pkg, &ri.version)?;
            let f = File::open(p)?;

            releases.push(serde_yaml::from_reader(f)?);
        }

        Ok(releases)
    }

    fn find(&self, _condition: &Self::Condition) -> Result<Vec<Self::ItemInstance>> {
        // TODO find by package name

        unimplemented!()
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
        let config = self.config.as_ref().unwrap();
        let container = di_container();
        let _pkg_service = container.get::<PackageService>().unwrap();

        let index_f = config.current_index_file()?;
        let index_f = File::open(index_f)?;
        let current_releases: Vec<ReleaseIndex> = serde_yaml::from_reader(index_f)?;

        let mut releases = self.list()?;

        for mut r in releases.iter_mut() {
            if name.is_some() && r.package.name != name.unwrap() {
                continue;
            }

            if current_releases
                .iter()
                .find(|it| it.version == r.version)
                .is_some()
            {
                r.is_current = true;
            }
        }

        Ok(releases)
    }
}

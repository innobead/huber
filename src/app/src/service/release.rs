use std::fs::File;
use std::path::PathBuf;
use std::sync::Arc;

use symlink::{remove_symlink_dir, symlink_dir};
use tokio::runtime::Runtime;

use huber_common::config::Config;
use huber_common::di::di_container;
use huber_common::model::package::Package;
use huber_common::model::release::{Release, ReleaseIndex};
use huber_common::result::Result;

use crate::service::package::PackageService;
use crate::service::{ItemOperationTrait, ItemSearchTrait};

pub(crate) trait ReleaseTrait {
    fn current(&self, pkg: &Package) -> Result<Release>;
    fn set_current(&self, release: &Release) -> Result<()>;
    fn list_current(&self) -> Result<Vec<Release>>;
    fn delete_release(&self, release: &Release) -> Result<()>;
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

    fn list_current(&self) -> Result<Vec<Release>> {
        let config = self.config.as_ref().unwrap();
        let f = config.current_index_file()?;
        let f = File::open(f)?;

        let container = di_container();
        let package_service = container.get::<PackageService>().unwrap();
        let mut releases: Vec<Release> = vec![];

        let indexes: Vec<ReleaseIndex> = serde_yaml::from_reader(f)?;
        for x in indexes {
            let pkg = package_service.get(&x.name)?;

            let p = config.installed_pkg_manifest_file(&pkg, &x.version)?;
            let f = File::open(p)?;
            releases.push(serde_yaml::from_reader(f)?);
        }

        Ok(releases)
    }

    fn delete_release(&self, release: &Release) -> Result<()> {
        let current_r = self.current(&release.package)?;

        if current_r.version == release.version {
            return Err(anyhow!(
                "{} is the current release, not able to delete!",
                release
            ));
        }

        let config = self.config.as_ref().unwrap();
        config
            .installed_pkg_dir(&release.package, &release.version)
            .map(|_| ())
    }
}

impl ItemOperationTrait for ReleaseService {
    type Item = Package;
    type ItemInstance = Release;
    type Condition = String;

    fn create(&self, _obj: &Self::Item) -> Result<Self::ItemInstance> {
        unimplemented!()
    }

    fn update(&self, _obj: &Self::Item) -> Result<Self::ItemInstance> {
        unimplemented!()
    }

    fn delete(&self, _name: &str) -> Result<()> {
        unimplemented!()
    }

    fn list(&self) -> Result<Vec<Self::ItemInstance>> {
        unimplemented!()
    }

    fn find(&self, _condition: &Self::Condition) -> Result<Vec<Self::ItemInstance>> {
        unimplemented!()
    }

    fn get(&self, _name: &str) -> Result<Self::ItemInstance> {
        unimplemented!()
    }

    fn has(&self, name: &str) -> Result<bool> {
        Ok(self
            .search(Some(name), None, None)
            .map(|_| true)
            .unwrap_or(false))
    }
}

impl ItemSearchTrait for ReleaseService {
    type Item = Release;

    fn search(
        &self,
        _name: Option<&str>,
        _pattern: Option<&str>,
        _owner: Option<&str>,
    ) -> Result<Vec<Self::Item>> {
        unimplemented!()
    }

    fn info(&self, _name: &str) -> Result<Self::Item> {
        unimplemented!()
    }
}

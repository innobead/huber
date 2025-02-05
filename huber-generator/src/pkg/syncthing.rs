use huber_common::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "syncthing".to_string(),
        source: PackageSource::Github {
            owner: "syncthing".to_string(),
            repo: "syncthing".to_string(),
        },
        ..Default::default()
    }
}

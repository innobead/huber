use huber_common::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "ripgrep".to_string(),
        source: PackageSource::Github {
            owner: "BurntSushi".to_string(),
            repo: "ripgrep".to_string(),
        },
        ..Default::default()
    }
}

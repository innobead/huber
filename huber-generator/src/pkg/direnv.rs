use huber_common::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "direnv".to_string(),
        source: PackageSource::Github {
            owner: "direnv".to_string(),
            repo: "direnv".to_string(),
        },
        ..Default::default()
    }
}

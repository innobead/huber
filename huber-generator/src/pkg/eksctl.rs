use huber_common::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "eksctl".to_string(),
        source: PackageSource::Github {
            owner: "eksctl-io".to_string(),
            repo: "eksctl".to_string(),
        },
        ..Default::default()
    }
}

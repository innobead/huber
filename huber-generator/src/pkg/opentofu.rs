use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "opentofu".to_string(),
        source: PackageSource::Github {
            owner: "opentofu".to_string(),
            repo: "opentofu".to_string(),
        },
        ..Default::default()
    }
}

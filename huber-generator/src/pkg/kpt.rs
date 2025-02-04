use huber_common::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "kpt".to_string(),
        source: PackageSource::Github {
            owner: "GoogleContainerTools".to_string(),
            repo: "kpt".to_string(),
        },
        ..Default::default()
    }
}

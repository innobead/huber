use huber_common::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "tilt".to_string(),
        source: PackageSource::Github {
            owner: "tilt-dev".to_string(),
            repo: "tilt".to_string(),
        },
        ..Default::default()
    }
}

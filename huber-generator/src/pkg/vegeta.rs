use huber_common::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "vegeta".to_string(),
        source: PackageSource::Github {
            owner: "tsenart".to_string(),
            repo: "vegeta".to_string(),
        },
        ..Default::default()
    }
}

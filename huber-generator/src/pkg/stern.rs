use huber_common::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "stern".to_string(),
        source: PackageSource::Github {
            owner: "stern".to_string(),
            repo: "stern".to_string(),
        },
        ..Default::default()
    }
}

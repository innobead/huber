use huber_common::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "ali".to_string(),
        source: PackageSource::Github {
            owner: "nakabonne".to_string(),
            repo: "ali".to_string(),
        },
        ..Default::default()
    }
}

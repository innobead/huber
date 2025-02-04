use huber_common::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "bun".to_string(),
        source: PackageSource::Github {
            owner: "oven-sh".to_string(),
            repo: "bun".to_string(),
        },
        ..Default::default()
    }
}

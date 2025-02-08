use huber::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "sd".to_string(),
        source: PackageSource::Github {
            owner: "chmln".to_string(),
            repo: "sd".to_string(),
        },
        ..Default::default()
    }
}

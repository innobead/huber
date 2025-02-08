use huber::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "lsd".to_string(),
        source: PackageSource::Github {
            owner: "lsd-rs".to_string(),
            repo: "lsd".to_string(),
        },
        ..Default::default()
    }
}

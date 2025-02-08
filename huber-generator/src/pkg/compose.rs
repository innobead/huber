use huber::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "compose".to_string(),
        source: PackageSource::Github {
            owner: "docker".to_string(),
            repo: "compose".to_string(),
        },
        ..Default::default()
    }
}

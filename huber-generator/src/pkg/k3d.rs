use huber::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "k3d".to_string(),
        source: PackageSource::Github {
            owner: "k3d-io".to_string(),
            repo: "k3d".to_string(),
        },
        ..Default::default()
    }
}

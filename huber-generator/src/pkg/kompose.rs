use huber::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "kompose".to_string(),
        source: PackageSource::Github {
            owner: "kubernetes".to_string(),
            repo: "kompose".to_string(),
        },
        ..Default::default()
    }
}

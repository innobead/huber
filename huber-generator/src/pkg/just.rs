use huber::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "just".to_string(),
        source: PackageSource::Github {
            owner: "casey".to_string(),
            repo: "just".to_string(),
        },
        ..Default::default()
    }
}

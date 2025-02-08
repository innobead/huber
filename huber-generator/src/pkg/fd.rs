use huber::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "fd".to_string(),
        source: PackageSource::Github {
            owner: "sharkdp".to_string(),
            repo: "fd".to_string(),
        },
        ..Default::default()
    }
}

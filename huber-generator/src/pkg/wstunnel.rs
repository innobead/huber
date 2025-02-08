use huber::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "wstunnel".to_string(),
        source: PackageSource::Github {
            owner: "erebe".to_string(),
            repo: "wstunnel".to_string(),
        },
        ..Default::default()
    }
}

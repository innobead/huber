use huber::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "bat".to_string(),
        source: PackageSource::Github {
            owner: "sharkdp".to_string(),
            repo: "bat".to_string(),
        },
        ..Default::default()
    }
}

use huber_common::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "hexyl".to_string(),
        source: PackageSource::Github {
            owner: "sharkdp".to_string(),
            repo: "hexyl".to_string(),
        },
        ..Default::default()
    }
}

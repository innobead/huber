use huber_common::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "k9s".to_string(),
        source: PackageSource::Github {
            owner: "derailed".to_string(),
            repo: "k9s".to_string(),
        },
        ..Default::default()
    }
}

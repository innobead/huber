use huber_common::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "cloak".to_string(),
        source: PackageSource::Github {
            owner: "cbeuw".to_string(),
            repo: "Cloak".to_string(),
        },
        ..Default::default()
    }
}

use huber_common::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "krew".to_string(),
        source: PackageSource::Github {
            owner: "kubernetes-sigs".to_string(),
            repo: "krew".to_string(),
        },
        ..Default::default()
    }
}

use huber_common::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "kustomize".to_string(),
        source: PackageSource::Github {
            owner: "kubernetes-sigs".to_string(),
            repo: "kustomize".to_string(),
        },
        ..Default::default()
    }
}

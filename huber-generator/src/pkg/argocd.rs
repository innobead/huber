use huber_common::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "argocd".to_string(),
        source: PackageSource::Github {
            owner: "argoproj".to_string(),
            repo: "argo-cd".to_string(),
        },
        ..Default::default()
    }
}

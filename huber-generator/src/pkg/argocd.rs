use huber::model::package::{default_targets_no_arm, Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "argocd".to_string(),
        source: PackageSource::Github {
            owner: "argoproj".to_string(),
            repo: "argo-cd".to_string(),
        },
        targets: default_targets_no_arm(),
        ..Default::default()
    }
}

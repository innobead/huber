use huber_common::model::package::{Package, PackageSource};

use crate::pkg::default_targets;

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "argocd".to_string(),
        source: PackageSource::Github {
            owner: "argoproj".to_string(),
            repo: "argo-cd".to_string(),
        },
        targets: default_targets(),
        ..Default::default()
    }
}

use huber_common::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "minikube".to_string(),
        source: PackageSource::Github {
            owner: "kubernetes".to_string(),
            repo: "minikube".to_string(),
        },
        ..Default::default()
    }
}

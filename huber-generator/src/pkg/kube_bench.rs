use huber::model::package::{default_targets_no_windows, Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "kube-bench".to_string(),
        source: PackageSource::Github {
            owner: "aquasecurity".to_string(),
            repo: "kube-bench".to_string(),
        },
        targets: default_targets_no_windows(),
        ..Default::default()
    }
}

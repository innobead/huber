use huber_common::model::package::{
    default_targets_no_arm, Package, PackageSource,
};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "k6".to_string(),
        source: PackageSource::Github {
            owner: "grafana".to_string(),
            repo: "k6".to_string(),
        },
        targets: default_targets_no_arm(),
        ..Default::default()
    }
}

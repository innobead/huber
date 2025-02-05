use huber_common::model::package::{default_targets_no_arm, Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "terrascan".to_string(),
        source: PackageSource::Github {
            owner: "tenable".to_string(),
            repo: "terrascan".to_string(),
        },
        targets: default_targets_no_arm(),
        ..Default::default()
    }
}

use huber_common::model::package::{default_targets_no_arm, Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "procs".to_string(),
        source: PackageSource::Github {
            owner: "dalance".to_string(),
            repo: "procs".to_string(),
        },
        targets: default_targets_no_arm(),
        ..Default::default()
    }
}

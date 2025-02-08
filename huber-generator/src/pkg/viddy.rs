use huber::model::package::{default_targets_no_arm, Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "viddy".to_string(),
        source: PackageSource::Github {
            owner: "sachaos".to_string(),
            repo: "viddy".to_string(),
        },
        targets: default_targets_no_arm(),
        ..Default::default()
    }
}

use huber::model::package::{default_targets_no_arm, Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "regclient".to_string(),
        source: PackageSource::Github {
            owner: "regclient".to_string(),
            repo: "regclient".to_string(),
        },
        targets: default_targets_no_arm(),
        ..Default::default()
    }
}

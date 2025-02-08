use huber::model::package::{default_targets_no_arm, Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "dolt".to_string(),
        source: PackageSource::Github {
            owner: "dolthub".to_string(),
            repo: "dolt".to_string(),
        },
        targets: default_targets_no_arm(),
        ..Default::default()
    }
}

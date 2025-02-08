use huber::model::package::{default_targets_no_arm, Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "goose".to_string(),
        source: PackageSource::Github {
            owner: "pressly".to_string(),
            repo: "goose".to_string(),
        },
        targets: default_targets_no_arm(),
        ..Default::default()
    }
}

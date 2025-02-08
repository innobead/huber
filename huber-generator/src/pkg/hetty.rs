use huber::model::package::{default_targets_no_arm, Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "hetty".to_string(),
        source: PackageSource::Github {
            owner: "dstotijn".to_string(),
            repo: "hetty".to_string(),
        },
        targets: default_targets_no_arm(),
        ..Default::default()
    }
}

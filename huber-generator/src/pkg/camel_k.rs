use huber::model::package::default_targets_no_arm;
use huber::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "camel-k".to_string(),
        source: PackageSource::Github {
            owner: "apache".to_string(),
            repo: "camel-k".to_string(),
        },
        targets: default_targets_no_arm(),
        ..Default::default()
    }
}

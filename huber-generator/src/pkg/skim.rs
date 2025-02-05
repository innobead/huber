use huber_common::model::package::{default_targets_no_windows, Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "skim".to_string(),
        source: PackageSource::Github {
            owner: "skim-rs".to_string(),
            repo: "skim".to_string(),
        },
        targets: default_targets_no_windows(),
        ..Default::default()
    }
}

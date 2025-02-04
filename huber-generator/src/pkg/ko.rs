use huber_common::model::package::{
    default_targets_no_arm, Package, PackageSource,
};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "ko".to_string(),
        source: PackageSource::Github {
            owner: "ko-build".to_string(),
            repo: "ko".to_string(),
        },
        targets: default_targets_no_arm(),
        ..Default::default()
    }
}

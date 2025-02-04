use huber_common::model::package::{
    default_targets_no_arm, Package, PackageSource,
};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "ollama".to_string(),
        source: PackageSource::Github {
            owner: "ollama".to_string(),
            repo: "ollama".to_string(),
        },
        targets: default_targets_no_arm(),
        ..Default::default()
    }
}

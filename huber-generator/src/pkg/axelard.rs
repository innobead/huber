use huber_common::model::package::{Package, PackageSource};

use crate::pkg::default_targets_no_arm_windows;

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "axelard".to_string(),
        source: PackageSource::Github {
            owner: "axelarnetwork".to_string(),
            repo: "axelar-core".to_string(),
        },
        targets: default_targets_no_arm_windows(),
        ..Default::default()
    }
}

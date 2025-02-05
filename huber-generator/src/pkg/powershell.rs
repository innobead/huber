use huber_common::model::package::{default_targets_no_arm, Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "powershell".to_string(),
        source: PackageSource::Github {
            owner: "PowerShell".to_string(),
            repo: "PowerShell".to_string(),
        },
        targets: default_targets_no_arm(),
        ..Default::default()
    }
}

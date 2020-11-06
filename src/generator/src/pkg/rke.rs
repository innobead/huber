use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "rke".to_string(),
        source: PackageSource::Github {
            owner: "rancher".to_string(),
            repo: "rke".to_string(),
        },
        detail: None,
        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["{version}/rke_linux-amd64".to_string()],
                checksum: None,
                install_commands: None,
                uninstall_commands: None,
                upgrade_commands: None,
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec!["{version}/rke_linux-arm64".to_string()],
                checksum: None,
                install_commands: None,
                uninstall_commands: None,
                upgrade_commands: None,
            }),
            PackageTargetType::MacOS(PackageManagement {
                artifact_templates: vec!["{version}/rke_darwin-arm64".to_string()],
                checksum: None,
                install_commands: None,
                uninstall_commands: None,
                upgrade_commands: None,
            }),
        ],
        version: None,
        description: None,
    }
}

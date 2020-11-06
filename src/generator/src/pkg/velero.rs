use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "velero".to_string(),
        source: PackageSource::Github {
            owner: "vmware-tanzu".to_string(),
            repo: "velero".to_string(),
        },
        detail: None,
        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["velero-{version}-linux-amd64.tar.gz".to_string()],
                checksum: Some("CHECKSUM".to_string()),
                install_commands: None,
                uninstall_commands: None,
                upgrade_commands: None,
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec!["velero-{version}-linux-arm64.tar.gz".to_string()],
                checksum: Some("CHECKSUM".to_string()),
                install_commands: None,
                uninstall_commands: None,
                upgrade_commands: None,
            }),
            PackageTargetType::MacOS(PackageManagement {
                artifact_templates: vec!["velero-{version}-darwin-arm64.tar.gz".to_string()],
                checksum: Some("CHECKSUM".to_string()),
                install_commands: None,
                uninstall_commands: None,
                upgrade_commands: None,
            }),
        ],
        version: None,
        description: None,
    }
}

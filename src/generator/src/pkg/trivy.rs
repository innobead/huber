use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "trivy".to_string(),
        source: PackageSource::Github {
            owner: "aquasecurity".to_string(),
            repo: "trivy".to_string(),
        },
        detail: None,
        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["trivy_{version}_Linux-64bit.tar.gz".to_string()],
                executable_templates: None,
                checksum: None,
                install_commands: None,
                uninstall_commands: None,
                upgrade_commands: None,
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec!["trivy_{version}_Linux-ARM64.tar.gz".to_string()],
                executable_templates: None,
                checksum: None,
                install_commands: None,
                uninstall_commands: None,
                upgrade_commands: None,
            }),
            PackageTargetType::MacOS(PackageManagement {
                artifact_templates: vec!["trivy_{version}_macOS-64bit.tar.gz".to_string()],
                executable_templates: None,
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

use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "go".to_string(),
        source: PackageSource::Github {
            owner: "golang".to_string(),
            repo: "go".to_string(),
        },
        detail: None,
        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec![
                    "https://golang.org/dl/go{version}.linux-amd64.tar.gz".to_string()
                ],
                executable_templates: None,
                checksum: None,
                install_commands: None,
                uninstall_commands: None,
                upgrade_commands: None,
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec![
                    "https://golang.org/dl/go{version}.linux-arm64.tar.gz".to_string()
                ],
                executable_templates: None,
                checksum: None,
                install_commands: None,
                uninstall_commands: None,
                upgrade_commands: None,
            }),
            PackageTargetType::MacOS(PackageManagement {
                artifact_templates: vec![
                    "https://golang.org/dl/go{version}.darwin-amd64.tar.gz".to_string()
                ],
                executable_templates: None,
                checksum: None,
                install_commands: None,
                uninstall_commands: None,
                upgrade_commands: None,
            }),
            PackageTargetType::Windows(PackageManagement {
                artifact_templates: vec![
                    "https://golang.org/dl/go{version}.windows-amd64.zip".to_string()
                ],
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

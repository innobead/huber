use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "kubefire".to_string(),
        source: PackageSource::Github {
            owner: "innobead".to_string(),
            repo: "kubefire".to_string(),
        },
        detail: None,
        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec![
                    "{version}/kubefire-linux-amd64".to_string(),
                    "{version}/host-local-rev-linux-amd64".to_string(),
                ],
                checksum: None,
                install_commands: None,
                uninstall_commands: None,
                upgrade_commands: None,
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec![
                    "{version}/kubefire-linux-arm64".to_string(),
                    "{version}/host-local-rev-linux-arm64".to_string(),
                ],
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

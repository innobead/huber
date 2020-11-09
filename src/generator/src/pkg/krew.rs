use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "krew".to_string(),
        source: PackageSource::Github {
            owner: "kubernetes-sigs".to_string(),
            repo: "krew".to_string(),
        },
        detail: None,
        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["{version}/krew.tar.gz".to_string()],
                executable_templates: Some(vec!["krew-linux_amd64".to_string()]),
                checksum: None,
                install_commands: None,
                uninstall_commands: None,
                upgrade_commands: None,
            }),
            PackageTargetType::MacOS(PackageManagement {
                artifact_templates: vec!["{version}/kind-darwin-amd64".to_string()],
                executable_templates: Some(vec!["krew-darwin_amd64".to_string()]),
                checksum: None,
                install_commands: None,
                uninstall_commands: None,
                upgrade_commands: None,
            }),
            PackageTargetType::Windows(PackageManagement {
                artifact_templates: vec!["{version}/krew.exe".to_string()],
                executable_templates: Some(vec!["krew-windows_amd64.exe".to_string()]),
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

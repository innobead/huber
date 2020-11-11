use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "fission".to_string(),
        source: PackageSource::Github {
            owner: "fission".to_string(),
            repo: "fission".to_string(),
        },
        detail: None,
        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec![
                    "{version}/fission-cli-linux".to_string(),
                    "fission-all-{version}.tgz".to_string(),
                    "fission-core-{version}.tgz".to_string(),
                ],
                executable_templates: None,
                executable_mappings: Some(hashmap! {
                    "fission-cli-linux".to_string() => "fission".to_string()
                }),
                install_commands: None,
                uninstall_commands: None,
                upgrade_commands: None,
            }),
            PackageTargetType::MacOS(PackageManagement {
                artifact_templates: vec![
                    "{version}/fission-cli-osx".to_string(),
                    "fission-all-{version}.tgz".to_string(),
                    "fission-core-{version}.tgz".to_string(),
                ],
                executable_templates: None,
                executable_mappings: Some(hashmap! {
                    "fission-cli-osx".to_string() => "fission".to_string()
                }),
                install_commands: None,
                uninstall_commands: None,
                upgrade_commands: None,
            }),
            PackageTargetType::Windows(PackageManagement {
                artifact_templates: vec![
                    "{version}/fission-cli-windows.exe".to_string(),
                    "fission-all-{version}.tgz".to_string(),
                    "fission-core-{version}.tgz".to_string(),
                ],
                executable_templates: None,
                executable_mappings: Some(hashmap! {
                    "fission-cli-windows.exe".to_string() => "fission.exe".to_string()
                }),
                install_commands: None,
                uninstall_commands: None,
                upgrade_commands: None,
            }),
        ],
        version: None,
        description: None,
        release_kind: None
    }
}

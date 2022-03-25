use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "hub".to_string(),
        source: PackageSource::Github {
            owner: "github".to_string(),
            repo: "hub".to_string(),
        },
        detail: None,
        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["hub-linux-amd64-{version}.tgz".to_string()],
                executable_templates: None,
                executable_mappings: Some(hashmap! {
                    "install".to_string() => "hub-install".to_string()
                }),
                install_commands: None,
                uninstall_commands: None,
                upgrade_commands: None,
                tag_version_regex_template: None,
                scan_dirs: None,
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec!["hub-linux-arm64-{version}.tgz".to_string()],
                executable_templates: None,
                executable_mappings: None,
                install_commands: None,
                uninstall_commands: None,
                upgrade_commands: None,
                tag_version_regex_template: None,
                scan_dirs: None,
            }),
            PackageTargetType::MacOS(PackageManagement {
                artifact_templates: vec!["hub-darwin-amd64-{version}.tgz".to_string()],
                executable_templates: None,
                executable_mappings: None,
                install_commands: None,
                uninstall_commands: None,
                upgrade_commands: None,
                tag_version_regex_template: None,
                scan_dirs: None,
            }),
            PackageTargetType::Windows(PackageManagement {
                artifact_templates: vec!["hub-windows-amd64-{version}.zip".to_string()],
                executable_templates: None,
                executable_mappings: None,
                install_commands: None,
                uninstall_commands: None,
                upgrade_commands: None,
                tag_version_regex_template: None,
                scan_dirs: None,
            }),
        ],
        version: None,
        description: None,
        release_kind: None,
    }
}

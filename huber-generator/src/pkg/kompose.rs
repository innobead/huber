use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "kompose".to_string(),
        source: PackageSource::Github {
            owner: "kubernetes".to_string(),
            repo: "kompose".to_string(),
        },
        detail: None,
        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["{version}/kompose-linux-arm64".to_string()],
                executable_templates: None,
                executable_mappings: None,
                install_commands: None,
                uninstall_commands: None,
                upgrade_commands: None,
                tag_version_regex_template: None,
                scan_dirs: None,
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec!["{version}/kompose-linux-amd64".to_string()],
                executable_templates: None,
                executable_mappings: None,
                install_commands: None,
                uninstall_commands: None,
                upgrade_commands: None,
                tag_version_regex_template: None,
                scan_dirs: None,
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["{version}/kompose-darwin-amd64".to_string()],
                executable_templates: None,
                executable_mappings: None,
                install_commands: None,
                uninstall_commands: None,
                upgrade_commands: None,
                tag_version_regex_template: None,
                scan_dirs: None,
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["{version}/kompose-windows-amd64.exe".to_string()],
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

use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "cosign".to_string(),
        source: PackageSource::Github {
            owner: "sigstore".to_string(),
            repo: "cosign".to_string(),
        },
        detail: None,
        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec![
                    "{version}/cosign-linux-amd64".to_string(),
                    "{version}/cosigned-linux-amd64".to_string(),
                    "{version}/sget-linux-amd64".to_string(),
                ],
                executable_templates: None,
                executable_mappings: None,
                install_commands: None,
                uninstall_commands: None,
                upgrade_commands: None,
                tag_version_regex_template: None,
                scan_dirs: None,
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec![
                    "{version}/cosign-linux-arm64".to_string(),
                    "{version}/cosigned-linux-arm64".to_string(),
                    "{version}/sget-linux-arm64".to_string(),
                ],
                executable_templates: None,
                executable_mappings: None,
                install_commands: None,
                uninstall_commands: None,
                upgrade_commands: None,
                tag_version_regex_template: None,
                scan_dirs: None,
            }),
            PackageTargetType::LinuxArm32(PackageManagement {
                artifact_templates: vec![
                    "{version}/cosign-linux-arm".to_string(),
                    "{version}/cosigned-linux-arm".to_string(),
                    "{version}/sget-linux-arm".to_string(),
                ],
                executable_templates: None,
                executable_mappings: None,
                install_commands: None,
                uninstall_commands: None,
                upgrade_commands: None,
                tag_version_regex_template: None,
                scan_dirs: None,
            }),
            PackageTargetType::MacOS(PackageManagement {
                artifact_templates: vec![
                    "{version}/cosign-darwin-amd64".to_string(),
                    "{version}/sget-darwin-amd64".to_string(),
                ],
                executable_templates: None,
                executable_mappings: None,
                install_commands: None,
                uninstall_commands: None,
                upgrade_commands: None,
                tag_version_regex_template: None,
                scan_dirs: None,
            }),
            PackageTargetType::MacOSArm64(PackageManagement {
                artifact_templates: vec![
                    "{version}/cosign-darwin-arm64".to_string(),
                    "{version}/sget-darwin-arm64".to_string(),
                ],
                executable_templates: None,
                executable_mappings: None,
                install_commands: None,
                uninstall_commands: None,
                upgrade_commands: None,
                tag_version_regex_template: None,
                scan_dirs: None,
            }),
            PackageTargetType::Windows(PackageManagement {
                artifact_templates: vec![
                    "{version}/cosign-windows-amd64.exe".to_string(),
                    "{version}/sget-windows-amd64.exe".to_string(),
                ],
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

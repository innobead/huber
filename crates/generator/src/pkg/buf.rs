use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "buf".to_string(),
        source: PackageSource::Github {
            owner: "bufbuild".to_string(),
            repo: "buf".to_string(),
        },
        detail: None,
        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec![
                    "{version}/buf-Linux-x86_64".to_string(),
                    "{version}/protoc-gen-buf-breaking-Linux-x86_64".to_string(),
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
                    "{version}/buf-Linux-aarch64".to_string(),
                    "{version}/protoc-gen-buf-breaking-Linux-aarch64".to_string(),
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
                    "{version}/buf-Darwin-x86_64".to_string(),
                    "{version}/protoc-gen-buf-breaking-Darwin-x86_64".to_string(),
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
                    "{version}/buf-Darwin-arm64".to_string(),
                    "{version}/protoc-gen-buf-breaking-Darwin-arm64".to_string(),
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
                    "{version}/buf-Windows-x86_64".to_string(),
                    "{version}/protoc-gen-buf-breaking-Windows-x86_64".to_string(),
                ],
                executable_templates: None,
                executable_mappings: None,
                install_commands: None,
                uninstall_commands: None,
                upgrade_commands: None,
                tag_version_regex_template: None,
                scan_dirs: None,
            }),
            PackageTargetType::WindowsArm64(PackageManagement {
                artifact_templates: vec![
                    "{version}/buf-Windows-arm64".to_string(),
                    "{version}/protoc-gen-buf-breaking-Windows-arm64".to_string(),
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

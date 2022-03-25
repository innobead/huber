use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "krustlet".to_string(),
        source: PackageSource::Github {
            owner: "deislabs".to_string(),
            repo: "krustlet".to_string(),
        },
        detail: None,
        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["https://krustlet.blob.core.windows.net/releases/krustlet-v{version}-linux-amd64.tar.gz".to_string()],
                executable_templates: None,
                executable_mappings: None,
                install_commands: None,
                uninstall_commands: None,
                upgrade_commands: None,
                tag_version_regex_template: None,
                scan_dirs: None,
            }),
            PackageTargetType::MacOS(PackageManagement {
                artifact_templates: vec!["https://krustlet.blob.core.windows.net/releases/krustlet-v{version}-macos-amd64.tar.gz".to_string()],
                executable_templates: None,
                executable_mappings: None,
                install_commands: None,
                uninstall_commands: None,
                upgrade_commands: None,
                tag_version_regex_template: None,
                scan_dirs: None,
            }),
            PackageTargetType::Windows(PackageManagement {
                artifact_templates: vec!["https://krustlet.blob.core.windows.net/releases/krustlet-v{version}-windows-amd64.tar.gz".to_string()],
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

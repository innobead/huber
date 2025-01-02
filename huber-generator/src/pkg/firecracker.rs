use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "firecracker".to_string(),
        source: PackageSource::Github {
            owner: "firecracker-microvm".to_string(),
            repo: "firecracker".to_string(),
        },
        detail: None,
        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec![
                    "Firecracker-v{version}-x86_64.tgz".to_string(),
                    "firecracker-v{version}-x86_64.tgz".to_string(),
                ],
                executable_templates: None,
                executable_mappings: None,
                install_commands: None,
                uninstall_commands: None,
                upgrade_commands: None,
                tag_version_regex_template: None,
                scan_dirs: Some(vec!["release-v{version}".to_string()]),
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec![
                    "Firecracker-v{version}-aarch64.tgz".to_string(),
                    "firecracker-v{version}-aarch64.tgz".to_string(),
                ],
                executable_templates: None,
                executable_mappings: None,
                install_commands: None,
                uninstall_commands: None,
                upgrade_commands: None,
                tag_version_regex_template: None,
                scan_dirs: Some(vec!["release-v{version}".to_string()]),
            }),
        ],
        version: None,
        description: None,
        release_kind: None,
    }
}

use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "trivy".to_string(),
        source: PackageSource::Github {
            owner: "aquasecurity".to_string(),
            repo: "trivy".to_string(),
        },
        detail: None,
        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["trivy_{version}_Linux-64bit.tar.gz".to_string()],
                executable_templates: None,
                install_commands: None,
                uninstall_commands: None,
                upgrade_commands: None,
                executable_mappings: None,
                tag_version_regex_template: None,
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec!["trivy_{version}_Linux-ARM64.tar.gz".to_string()],
                executable_templates: None,
                executable_mappings: None,
                install_commands: None,
                uninstall_commands: None,
                upgrade_commands: None,
                tag_version_regex_template: None,
            }),
            PackageTargetType::MacOS(PackageManagement {
                artifact_templates: vec!["trivy_{version}_macOS-64bit.tar.gz".to_string()],
                executable_templates: None,
                install_commands: None,
                uninstall_commands: None,
                upgrade_commands: None,
                executable_mappings: None,
                tag_version_regex_template: None,
            }),
        ],
        version: None,
        description: None,
        release_kind: None,
    }
}

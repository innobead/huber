use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "ssvm".to_string(),
        source: PackageSource::Github {
            owner: "second-state".to_string(),
            repo: "SSVM".to_string(),
        },
        detail: None,
        targets: vec![PackageTargetType::LinuxAmd64(PackageManagement {
            artifact_templates: vec!["ssvm-{version}-linux-x64.tar.gz".to_string()],
            executable_templates: None,
            executable_mappings: None,
            install_commands: None,
            uninstall_commands: None,
            upgrade_commands: None,
            tag_version_regex_template: None,
        })],
        version: None,
        description: None,
        release_kind: None,
    }
}

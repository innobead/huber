use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "containerd".to_string(),
        source: PackageSource::Github {
            owner: "containerd".to_string(),
            repo: "containerd".to_string(),
        },
        detail: None,
        targets: vec![PackageTargetType::LinuxAmd64(PackageManagement {
            artifact_templates: vec!["containerd-{version}.linux-amd64.tar.gz".to_string()],
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

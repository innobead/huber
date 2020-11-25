use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "tracee".to_string(),
        source: PackageSource::Github {
            owner: "aquasecurity".to_string(),
            repo: "tracee".to_string(),
        },
        detail: None,
        targets: vec![PackageTargetType::LinuxAmd64(PackageManagement {
            artifact_templates: vec!["{version}/tracee.tar.gz".to_string()],
            executable_templates: None,
            executable_mappings: None,
            install_commands: None,
            uninstall_commands: None,
            upgrade_commands: None,
        })],
        version: None,
        description: None,
        release_kind: None,
    }
}

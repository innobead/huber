use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "kwctl".to_string(),
        source: PackageSource::Github {
            owner: "kubewarden".to_string(),
            repo: "kwctl".to_string(),
        },
        detail: None,
        targets: vec![PackageTargetType::LinuxAmd64(PackageManagement {
            artifact_templates: vec!["{version}/kwctl-linux-amd64.zip".to_string()],
            executable_templates: None,
            executable_mappings: None,
            install_commands: None,
            uninstall_commands: None,
            upgrade_commands: None,
            tag_version_regex_template: None,
            scan_dirs: None,
        })],
        version: None,
        description: None,
        release_kind: None,
    }
}

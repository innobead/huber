use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "deno".to_string(),
        source: PackageSource::Github {
            owner: "denoland".to_string(),
            repo: "deno".to_string(),
        },
        detail: None,
        targets: vec![PackageTargetType::LinuxAmd64(PackageManagement {
            artifact_templates: vec!["typescript-{version}.tgz".to_string()],
            checksum: None,
            install_commands: None,
            uninstall_commands: None,
            upgrade_commands: None,
        })],
        version: None,
        description: None,
    }
}

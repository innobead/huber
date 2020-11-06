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
            checksum: None,
            install_commands: None,
            uninstall_commands: None,
            upgrade_commands: None,
        })],
        version: None,
        description: None,
    }
}

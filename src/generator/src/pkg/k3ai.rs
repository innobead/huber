use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "k3ai".to_string(),
        source: PackageSource::Github {
            owner: "kf5i".to_string(),
            repo: "k3ai".to_string(),
        },
        detail: None,
        targets: vec![PackageTargetType::LinuxAmd64(PackageManagement {
            artifact_templates: vec!["{version}/k3sup".to_string()],
            executable_templates: None,
            executable_mappings: Some(hashmap! {
                "install".to_string() => "k3ai-install".to_string(),
            }),
            install_commands: None,
            uninstall_commands: None,
            upgrade_commands: None,
        })],
        version: None,
        description: None,
    }
}

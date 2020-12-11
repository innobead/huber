use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "ketch".to_string(),
        source: PackageSource::Github {
            owner: "shipa-corp".to_string(),
            repo: "ketch".to_string(),
        },
        detail: None,
        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["{version}/ketch-linux-amd64".to_string()],
                executable_templates: None,
                executable_mappings: None,
                install_commands: None,
                uninstall_commands: None,
                upgrade_commands: None,
            }),
            PackageTargetType::MacOS(PackageManagement {
                artifact_templates: vec!["{version}/ketch-darwin-amd64".to_string()],
                executable_templates: None,
                install_commands: None,
                uninstall_commands: None,
                upgrade_commands: None,
                executable_mappings: None,
            }),
        ],
        version: None,
        description: None,
        release_kind: None,
    }
}

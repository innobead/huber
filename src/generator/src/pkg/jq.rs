use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "jq".to_string(),
        source: PackageSource::Github {
            owner: "stedolan".to_string(),
            repo: "jq".to_string(),
        },
        detail: None,
        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["{version}/jq-linux64".to_string()],
                executable_templates: None,
                executable_mappings: Some(hashmap! {
                    "jq-linux64".to_string() => "jq".to_string()
                }),
                install_commands: None,
                uninstall_commands: None,
                upgrade_commands: None,
            }),
            PackageTargetType::MacOS(PackageManagement {
                artifact_templates: vec!["{version}/jq-osx-amd64".to_string()],
                executable_templates: None,
                executable_mappings: Some(hashmap! {
                    "jq-osx-amd64".to_string() => "jq".to_string()
                }),
                install_commands: None,
                uninstall_commands: None,
                upgrade_commands: None,
            }),
            PackageTargetType::Windows(PackageManagement {
                artifact_templates: vec!["{version}/jq-win64.exe".to_string()],
                executable_templates: None,
                executable_mappings: Some(hashmap! {
                    "jq-win64.exe".to_string() => "jq.exe".to_string()
                }),
                install_commands: None,
                uninstall_commands: None,
                upgrade_commands: None,
            }),
        ],
        version: None,
        description: None,
        release_kind: None,
    }
}

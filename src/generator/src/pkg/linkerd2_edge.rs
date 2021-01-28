use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "linkerd2-edge".to_string(),
        source: PackageSource::Github {
            owner: "linkerd".to_string(),
            repo: "linkerd2".to_string(),
        },
        detail: None,
        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["linkerd2-cli-edge-{version}-linux-amd64".to_string()],
                executable_templates: None,
                executable_mappings: Some(hashmap! {
                    "linkerd2-cli-edge-{version}-linux-amd64".to_string() => "linkerd2-edge".to_string()
                }),
                install_commands: None,
                uninstall_commands: None,
                upgrade_commands: None,
                tag_version_regex_template: Some(r"^edge-(\d+.\d+.\d+)$".to_string()),
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec!["linkerd2-cli-edge-{version}-linux-arm64".to_string()],
                executable_templates: None,
                executable_mappings: Some(hashmap! {
                    "linkerd2-cli-edge-{version}-linux-arm64".to_string() => "linkerd2-edge".to_string()
                }),
                install_commands: None,
                uninstall_commands: None,
                upgrade_commands: None,
                tag_version_regex_template: Some(r"^edge-(\d+.\d+.\d+)$".to_string()),
            }),
            PackageTargetType::MacOS(PackageManagement {
                artifact_templates: vec!["linkerd2-cli-edge-{version}-darwin".to_string()],
                executable_templates: None,
                executable_mappings: Some(hashmap! {
                    "linkerd2-cli-edge-{version}-darwin".to_string() => "linkerd2-edge".to_string()
                }),
                install_commands: None,
                uninstall_commands: None,
                upgrade_commands: None,
                tag_version_regex_template: Some(r"^edge-(\d+.\d+.\d+)$".to_string()),
            }),
            PackageTargetType::Windows(PackageManagement {
                artifact_templates: vec!["linkerd2-cli-edge-{version}-windows.exe".to_string()],
                executable_templates: None,
                executable_mappings: Some(hashmap! {
                    "linkerd2-cli-edge-{version}-windows.exe".to_string() => "linkerd2-edge.exe".to_string()
                }),
                install_commands: None,
                uninstall_commands: None,
                upgrade_commands: None,
                tag_version_regex_template: Some(r"^edge-(\d+.\d+.\d+)$".to_string()),
            }),
        ],
        version: None,
        description: None,
        release_kind: None,
    }
}

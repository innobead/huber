use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};
use maplit::hashmap;

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "xh".to_string(),
        source: PackageSource::Github {
            owner: "ducaale".to_string(),
            repo: "xh".to_string(),
        },
        detail: None,
        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec![
                    "xh-v{version}-x86_64-unknown-linux-musl.tar.gz".to_string()
                ],
                executable_templates: None,
                executable_mappings: Some(hashmap! {
                    "xh".to_string() => "xh xhs".to_string(),
                }),
                install_commands: None,
                uninstall_commands: None,
                upgrade_commands: None,
                tag_version_regex_template: None,
                scan_dirs: None,
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["xh-v{version}-x86_64-apple-darwin.tar.gz".to_string()],
                executable_templates: None,
                executable_mappings: Some(hashmap! {
                    "xh".to_string() => "xh xhs".to_string(),
                }),
                install_commands: None,
                uninstall_commands: None,
                upgrade_commands: None,
                tag_version_regex_template: None,
                scan_dirs: None,
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["xh-v{version}-x86_64-pc-windows-msvc.zip".to_string()],
                executable_templates: None,
                executable_mappings: Some(hashmap! {
                    "xh".to_string() => "xh xhs".to_string(),
                }),
                install_commands: None,
                uninstall_commands: None,
                upgrade_commands: None,
                tag_version_regex_template: None,
                scan_dirs: None,
            }),
        ],
        version: None,
        description: None,
        release_kind: None,
    }
}

use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "hyperfine".to_string(),
        source: PackageSource::Github {
            owner: "sharkdp".to_string(),
            repo: "hyperfine".to_string(),
        },
        detail: None,
        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec![
                    "hyperfine-v{version}-x86_64-unknown-linux-musl.tar.gz".to_string()
                ],
                executable_templates: None,
                executable_mappings: None,
                install_commands: None,
                uninstall_commands: None,
                upgrade_commands: None,
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec![
                    "hyperfine-v{version}-arm-unknown-linux-musleabihf.tar.gz".to_string(),
                ],
                executable_templates: None,
                executable_mappings: None,
                install_commands: None,
                uninstall_commands: None,
                upgrade_commands: None,
            }),
            PackageTargetType::MacOS(PackageManagement {
                artifact_templates: vec![
                    "hyperfine-v{version}-x86_64-apple-darwin.tar.gz".to_string()
                ],
                executable_templates: None,
                executable_mappings: None,
                install_commands: None,
                uninstall_commands: None,
                upgrade_commands: None,
            }),
            PackageTargetType::Windows(PackageManagement {
                artifact_templates: vec![
                    "hyperfine-v{version}-x86_64-pc-windows-msvc.zip".to_string()
                ],
                executable_templates: None,
                executable_mappings: None,
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

use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "powershell".to_string(),
        source: PackageSource::Github {
            owner: "PowerShell".to_string(),
            repo: "PowerShell".to_string(),
        },
        detail: None,
        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["powershell-{version}-linux-x64.tar.gz".to_string()],
                executable_templates: None,
                executable_mappings: None,
                install_commands: None,
                uninstall_commands: None,
                upgrade_commands: None,
            }),
            PackageTargetType::MacOS(PackageManagement {
                artifact_templates: vec!["powershell-{version}-osx-x64.tar.gz".to_string()],
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

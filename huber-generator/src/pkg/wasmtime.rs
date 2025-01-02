use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "wasmtime".to_string(),
        source: PackageSource::Github {
            owner: "bytecodealliance".to_string(),
            repo: "wasmtime".to_string(),
        },
        detail: None,
        targets: vec![
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["wasmtime-v{version}-x86_64-windows.zip".to_string()],
                executable_templates: None,
                executable_mappings: None,
                install_commands: None,
                uninstall_commands: None,
                upgrade_commands: None,
                tag_version_regex_template: None,
                scan_dirs: None,
            }),
            PackageTargetType::Default(PackageManagement {
                artifact_templates: vec!["wasmtime-v{version}-{arch}-{os}.tar.xz".to_string()],
                executable_templates: None,
                executable_mappings: None,
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

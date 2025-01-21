use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "wasmtime".to_string(),
        source: PackageSource::Github {
            owner: "bytecodealliance".to_string(),
            repo: "wasmtime".to_string(),
        },

        targets: vec![
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["wasmtime-v{version}-x86_64-windows.zip".to_string()],
                ..Default::default()
            }),
            PackageTargetType::Default(PackageManagement {
                artifact_templates: vec!["wasmtime-v{version}-{arch}-{os}.tar.xz".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}

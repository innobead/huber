use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "wasm-to-oci".to_string(),
        source: PackageSource::Github {
            owner: "engineerd".to_string(),
            repo: "wasm-to-oci".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["{version}/linux-amd64-wasm-to-oci".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["{version}/darwin-amd64-wasm-to-oci".to_string()],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["{version}/windows-amd64-wasm-to-oci.exe".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}

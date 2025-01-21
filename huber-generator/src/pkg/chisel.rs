use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "chisel".to_string(),
        source: PackageSource::Github {
            owner: "jpillora".to_string(),
            repo: "chisel".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["chisel_{version}_linux_amd64.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec!["chisel_{version}_linux_arm64.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["chisel_{version}_darwin_amd64.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["chisel_{version}_windows_amd64.gz".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}

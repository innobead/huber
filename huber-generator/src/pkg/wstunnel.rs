use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "wstunnel".to_string(),
        source: PackageSource::Github {
            owner: "erebe".to_string(),
            repo: "wstunnel".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["{version}/wstunnel-x64-linux.zip".to_string()],
                ..Default::default()
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec!["{version}/wstunnel-aarch64-ubuntu18.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["{version}/wstunnel-x64-macos.zip".to_string()],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["{version}/wstunnel-x64-windows.exe.zip".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}

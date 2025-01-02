use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "flux2".to_string(),
        source: PackageSource::Github {
            owner: "fluxcd".to_string(),
            repo: "flux2".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["flux_0.3.0_linux_amd64.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec!["flux_0.3.0_linux_arm64.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["flux_0.3.0_darwin_amd64.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["flux_0.3.0_windows_amd64.zip".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}

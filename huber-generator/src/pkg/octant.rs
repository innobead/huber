use huber::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "octant".to_string(),
        source: PackageSource::Github {
            owner: "vmware-tanzu".to_string(),
            repo: "octant".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["octant_{version}_Linux-64bit.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec!["octant_{version}_Linux-arm64.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["octant_{version}_macOS-64bit.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["octant_{version}_Windows-64bit.tar.gz".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}

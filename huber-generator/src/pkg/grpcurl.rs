use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "grpcurl".to_string(),
        source: PackageSource::Github {
            owner: "fullstorydev".to_string(),
            repo: "grpcurl".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["grpcurl_{version}_linux_x86_64.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec!["grpcurl_{version}_linux_arm64.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["grpcurl_{version}_osx_x86_64.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSArm64(PackageManagement {
                artifact_templates: vec!["grpcurl_{version}_osx_arm64.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["grpcurl_{version}_windows_x86_64.zip".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}

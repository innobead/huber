use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "protoc".to_string(),
        source: PackageSource::Github {
            owner: "protocolbuffers".to_string(),
            repo: "protobuf".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["protoc-{version}-linux-x86_64.zip".to_string()],
                ..Default::default()
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec!["protoc-{version}-linux-aarch_64.zip".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["protoc-{version}-osx-x86_64.zip".to_string()],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["protoc-{version}-win64.zip".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}

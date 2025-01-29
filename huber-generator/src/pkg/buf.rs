use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "buf".to_string(),
        source: PackageSource::Github {
            owner: "bufbuild".to_string(),
            repo: "buf".to_string(),
        },
        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec![
                    "buf-Linux-x86_64".to_string(),
                    "protoc-gen-buf-breaking-Linux-x86_64".to_string(),
                ],
                ..Default::default()
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec![
                    "buf-Linux-aarch64".to_string(),
                    "protoc-gen-buf-breaking-Linux-aarch64".to_string(),
                ],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec![
                    "buf-Darwin-x86_64".to_string(),
                    "protoc-gen-buf-breaking-Darwin-x86_64".to_string(),
                ],
                ..Default::default()
            }),
            PackageTargetType::MacOSArm64(PackageManagement {
                artifact_templates: vec![
                    "buf-Darwin-arm64".to_string(),
                    "protoc-gen-buf-breaking-Darwin-arm64".to_string(),
                ],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec![
                    "buf-Windows-x86_64".to_string(),
                    "protoc-gen-buf-breaking-Windows-x86_64".to_string(),
                ],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}

use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "node".to_string(),
        source: PackageSource::Github {
            owner: "nodejs".to_string(),
            repo: "node".to_string(),
        },
        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec![
                    "https://nodejs.org/dist/v{version}/node-v{version}-linux-x64.tar.xz"
                        .to_string(),
                ],
                ..Default::default()
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec![
                    "https://nodejs.org/dist/v{version}/node-v{version}-linux-arm64.tar.xz"
                        .to_string(),
                ],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec![
                    "https://nodejs.org/dist/v{version}/node-v{version}-darwin-x64.tar.gz"
                        .to_string(),
                ],
                ..Default::default()
            }),
            PackageTargetType::MacOSArm64(PackageManagement {
                artifact_templates: vec![
                    "https://nodejs.org/dist/v{version}/node-v{version}-darwin-arm64.tar.gz"
                        .to_string(),
                ],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec![
                    "https://nodejs.org/dist/v{version}/node-v{version}-win-x64.zip".to_string(),
                ],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}

use huber::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "pulumi".to_string(),
        source: PackageSource::Github {
            owner: "pulumi".to_string(),
            repo: "pulumi".to_string(),
        },
        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec![
                    "https://get.pulumi.com/releases/sdk/pulumi-v{version}-linux-x64.tar.gz"
                        .to_string(),
                ],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec![
                    "https://get.pulumi.com/releases/sdk/pulumi-v{version}-darwin-x64.tar.gz"
                        .to_string(),
                ],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec![
                    "https://get.pulumi.com/releases/sdk/pulumi-v{version}-windows-x64.zip"
                        .to_string(),
                ],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}

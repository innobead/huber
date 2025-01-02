use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "doctl".to_string(),
        source: PackageSource::Github {
            owner: "digitalocean".to_string(),
            repo: "doctl".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["doctl-{version}-linux-amd64.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec!["doctl-{version}-linux-arm64.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["doctl-{version}-darwin-amd64.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["doctl-{version}-windows-amd64.zip".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}

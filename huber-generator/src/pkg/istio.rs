use huber::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "istio".to_string(),
        source: PackageSource::Github {
            owner: "istio".to_string(),
            repo: "istio".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec![
                    "istio-{version}-linux-amd64.tar.gz".to_string(),
                    "istioctl-{version}-linux-amd64.tar.gz".to_string(),
                ],
                ..Default::default()
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec![
                    "istio-{version}-linux-arm64.tar.gz".to_string(),
                    "istioctl-{version}-linux-arm64.tar.gz".to_string(),
                ],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec![
                    "istio-{version}-osx.tar.gz".to_string(),
                    "istioctl-{version}-osx.tar.gz".to_string(),
                ],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec![
                    "istio-{version}-win.zip".to_string(),
                    "istioctl-{version}-win.zip".to_string(),
                ],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}

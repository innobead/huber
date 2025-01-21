use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "kubestr".to_string(),
        source: PackageSource::Github {
            owner: "kastenhq".to_string(),
            repo: "kubestr".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["kubestr-v{version}-linux-amd64.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["kubestr-v{version}-darwin-amd64.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["kubestr-v{version}-windows-amd64.zip".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}

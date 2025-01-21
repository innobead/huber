use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "containerd".to_string(),
        source: PackageSource::Github {
            owner: "containerd".to_string(),
            repo: "containerd".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec![
                    "containerd-{version}-linux-amd64.tar.gz".to_string(),
                    "cri-containerd-cni-{version}-linux-amd64.tar.gz".to_string(),
                ],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec![
                    "containerd-{version}-windows-amd64.tar.gz".to_string(),
                    "cri-containerd-cni-{version}-windows-amd64.tar.gz".to_string(),
                ],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}

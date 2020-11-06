use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "istio".to_string(),
        source: PackageSource::Github {
            owner: "istio".to_string(),
            repo: "istio".to_string(),
        },
        detail: None,
        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec![
                    "istio-{version}-linux-amd64.tar.gz".to_string(),
                    "istioctl-{version}-linux-amd64.tar.gz".to_string(),
                ],
                checksum: None,
                install_commands: None,
                uninstall_commands: None,
                upgrade_commands: None,
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec![
                    "istio-{version}-linux-arm64.tar.gz".to_string(),
                    "istioctl-{version}-linux-arm64.tar.gz".to_string(),
                ],
                checksum: None,
                install_commands: None,
                uninstall_commands: None,
                upgrade_commands: None,
            }),
            PackageTargetType::MacOS(PackageManagement {
                artifact_templates: vec![
                    "istio-{version}-osx.tar.gz".to_string(),
                    "istioctl-{version}-osx.tar.gz".to_string(),
                ],
                checksum: None,
                install_commands: None,
                uninstall_commands: None,
                upgrade_commands: None,
            }),
        ],
        version: None,
    }
}

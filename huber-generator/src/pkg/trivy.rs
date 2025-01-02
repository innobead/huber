use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "trivy".to_string(),
        source: PackageSource::Github {
            owner: "aquasecurity".to_string(),
            repo: "trivy".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["trivy_{version}_Linux-64bit.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::LinuxArm64(PackageManagement {
                artifact_templates: vec!["trivy_{version}_Linux-ARM64.tar.gz".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["trivy_{version}_macOS-64bit.tar.gz".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}

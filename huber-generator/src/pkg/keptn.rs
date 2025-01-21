use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "keptn".to_string(),
        source: PackageSource::Github {
            owner: "keptn".to_string(),
            repo: "keptn".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["{version}_keptn-linux.tar".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["{version}_keptn-macOS.tar".to_string()],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["{version}_keptn-windows.tar".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}

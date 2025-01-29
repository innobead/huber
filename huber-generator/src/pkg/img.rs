use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "img".to_string(),
        source: PackageSource::Github {
            owner: "genuinetools".to_string(),
            repo: "img".to_string(),
        },

        targets: vec![PackageTargetType::LinuxAmd64(PackageManagement {
            artifact_templates: vec!["img-linux-amd64".to_string()],
            ..Default::default()
        })],
        ..Default::default()
    }
}

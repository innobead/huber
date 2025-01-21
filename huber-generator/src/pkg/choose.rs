use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "choose".to_string(),
        source: PackageSource::Github {
            owner: "theryangeary".to_string(),
            repo: "choose".to_string(),
        },

        targets: vec![PackageTargetType::LinuxAmd64(PackageManagement {
            artifact_templates: vec!["{version}/choose".to_string()],
            ..Default::default()
        })],
        ..Default::default()
    }
}

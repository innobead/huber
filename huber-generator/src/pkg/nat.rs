use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "nat".to_string(),
        source: PackageSource::Github {
            owner: "willdoescode".to_string(),
            repo: "nat".to_string(),
        },

        targets: vec![PackageTargetType::MacOSAmd64(PackageManagement {
            artifact_templates: vec!["{version}/natls_osx_binary".to_string()],
            ..Default::default()
        })],
        ..Default::default()
    }
}

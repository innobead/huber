use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "ssvm".to_string(),
        source: PackageSource::Github {
            owner: "second-state".to_string(),
            repo: "SSVM".to_string(),
        },

        targets: vec![PackageTargetType::LinuxAmd64(PackageManagement {
            artifact_templates: vec!["ssvm-{version}-linux-x64.tar.gz".to_string()],
            ..Default::default()
        })],
        ..Default::default()
    }
}

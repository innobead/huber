use huber::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "typescript".to_string(),
        source: PackageSource::Github {
            owner: "microsoft".to_string(),
            repo: "TypeScript".to_string(),
        },
        targets: vec![PackageTargetType::LinuxAmd64(PackageManagement {
            artifact_templates: vec!["typescript-{version}.tgz".to_string()],
            ..Default::default()
        })],
        ..Default::default()
    }
}

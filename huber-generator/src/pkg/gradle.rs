use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "gradle".to_string(),
        source: PackageSource::Github {
            owner: "gradle".to_string(),
            repo: "gradle".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec![
                    "https://services.gradle.org/distributions/gradle-{version}-bin.zip"
                        .to_string(),
                ],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec![
                    "https://services.gradle.org/distributions/gradle-{version}-bin.zip"
                        .to_string(),
                ],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec![
                    "https://services.gradle.org/distributions/gradle-{version}-bin.zip"
                        .to_string(),
                ],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}

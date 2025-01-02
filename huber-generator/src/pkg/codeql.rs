use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "codeql".to_string(),
        source: PackageSource::Github {
            owner: "github".to_string(),
            repo: "codeql-cli-binaries".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec!["codeql-linux64.zip".to_string()],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec!["codeql-osx64.zip".to_string()],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec!["codeql-win64.zip".to_string()],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}

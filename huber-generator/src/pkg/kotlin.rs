use huber_common::model::package::{Package, PackageManagement, PackageSource, PackageTargetType};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "kotlin".to_string(),
        source: PackageSource::Github {
            owner: "JetBrains".to_string(),
            repo: "kotlin".to_string(),
        },

        targets: vec![
            PackageTargetType::LinuxAmd64(PackageManagement {
                artifact_templates: vec![
                    "kotlin-compiler-{version}.zip".to_string(),
                    //TODO add back when kotlinc removed
                    // "kotlin-native-linux-{version}.tar.gz".to_string(),
                ],
                ..Default::default()
            }),
            PackageTargetType::MacOSAmd64(PackageManagement {
                artifact_templates: vec![
                    "kotlin-compiler-{version}.zip".to_string(),
                    //"kotlin-native-macos-{version}.tar.gz".to_string(),
                ],
                ..Default::default()
            }),
            PackageTargetType::WindowsAmd64(PackageManagement {
                artifact_templates: vec![
                    "kotlin-compiler-{version}.zip".to_string(),
                    //"kotlin-native-windows-{version}.zip".to_string(),
                ],
                ..Default::default()
            }),
        ],
        ..Default::default()
    }
}

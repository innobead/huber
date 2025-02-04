use huber_common::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "copilot-cli".to_string(),
        source: PackageSource::Github {
            owner: "aws".to_string(),
            repo: "copilot-cli".to_string(),
        },
        ..Default::default()
    }
}

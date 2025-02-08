use huber::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "tokei".to_string(),
        source: PackageSource::Github {
            owner: "XAMPPRocky".to_string(),
            repo: "tokei".to_string(),
        },
        ..Default::default()
    }
}

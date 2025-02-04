use huber_common::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "dua-cli".to_string(),
        source: PackageSource::Github {
            owner: "Byron".to_string(),
            repo: "dua-cli".to_string(),
        },
        ..Default::default()
    }
}

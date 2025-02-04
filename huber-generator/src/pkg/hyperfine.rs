use huber_common::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "hyperfine".to_string(),
        source: PackageSource::Github {
            owner: "sharkdp".to_string(),
            repo: "hyperfine".to_string(),
        },
        ..Default::default()
    }
}

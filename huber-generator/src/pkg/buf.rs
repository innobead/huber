use huber::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "buf".to_string(),
        source: PackageSource::Github {
            owner: "bufbuild".to_string(),
            repo: "buf".to_string(),
        },
        ..Default::default()
    }
}

use huber::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "dust".to_string(),
        source: PackageSource::Github {
            owner: "bootandy".to_string(),
            repo: "dust".to_string(),
        },
        ..Default::default()
    }
}

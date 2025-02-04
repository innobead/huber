use huber_common::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "grpcurl".to_string(),
        source: PackageSource::Github {
            owner: "fullstorydev".to_string(),
            repo: "grpcurl".to_string(),
        },
        ..Default::default()
    }
}

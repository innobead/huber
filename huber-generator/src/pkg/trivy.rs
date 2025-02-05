use huber_common::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "trivy".to_string(),
        source: PackageSource::Github {
            owner: "aquasecurity".to_string(),
            repo: "trivy".to_string(),
        },
        ..Default::default()
    }
}

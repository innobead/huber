use huber_common::model::package::{Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "saml2aws".to_string(),
        source: PackageSource::Github {
            owner: "Versent".to_string(),
            repo: "saml2aws".to_string(),
        },
        ..Default::default()
    }
}

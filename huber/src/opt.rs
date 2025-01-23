use anyhow::anyhow;
use semver::{Version, VersionReq};

/// Parse package name and version
///
/// # Examples
///
/// ```
/// use huber::opt::parse_pkg_name_semver;
/// let (name, version) = parse_pkg_name_semver("package-name@1.2.3").unwrap();
/// assert_eq!(name, "package-name");
/// assert_eq!(version, "v1.2.3");
/// ```
pub fn parse_pkg_name_semver(name_version: &str) -> anyhow::Result<(String, String)> {
    let result: Vec<_> = name_version.splitn(2, '@').collect();

    if result.len() != 2 {
        return Err(anyhow!(
            "Failed to parse package name version due to invalid format"
        ));
    }

    let (name, version) = (result[0].to_string(), result[1].to_string());
    Version::parse(version.trim_start_matches('v'))?;

    Ok((name, version))
}
pub fn parse_pkg_name_optional_semver(name_version: &str) -> anyhow::Result<(String, String)> {
    let result: Vec<_> = name_version.splitn(2, '@').collect();

    if result.len() != 2 {
        return Ok((result[0].to_string(), "".to_string()));
    }

    let (name, version) = (result[0].to_string(), result[1].to_string());
    Version::parse(version.trim_start_matches('v'))?;

    Ok((name, version))
}

/// Parse package name and version requirement
///
/// # Examples
///
/// ```
/// use huber::opt::parse_pkg_name_semver_req;
/// let (name, version) = parse_pkg_name_semver_req("package-name@~1.2.3").unwrap();
/// assert_eq!(name, "package-name");
/// assert_eq!(version, "~1.2.3");
///
/// let (name, version) = parse_pkg_name_semver_req("package-name@1.2.3").unwrap();
/// assert_eq!(name, "package-name");
/// assert_eq!(version, "v1.2.3");
/// ```
pub fn parse_pkg_name_semver_req(name_version: &str) -> anyhow::Result<(String, String)> {
    let result: Vec<_> = name_version.splitn(2, '@').collect();

    if result.len() != 2 {
        return Err(anyhow!(
            "Failed to parse package name version due to invalid format"
        ));
    }

    let (name, version) = (result[0].to_string(), result[1].to_string());
    if Version::parse(&version).is_ok() {
        return Ok((name, version));
    }

    VersionReq::parse(&version)?;
    Ok((name, version))
}

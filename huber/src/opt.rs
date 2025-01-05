use anyhow::anyhow;

/// Parse package name and version
///
/// # Examples
///
/// ```
/// use huber::opt::parse_pkg_name_version;
/// let (name, version) = parse_pkg_name_version("package-name@version").unwrap();
/// assert_eq!(name, "package-name");
/// assert_eq!(version, "version");
/// ```
pub fn parse_pkg_name_version(name_version: &str) -> anyhow::Result<(String, String)> {
    let result: Vec<_> = name_version.splitn(2, '@').collect();

    if result.len() != 2 {
        return Err(anyhow!(
            "Failed to parse package name version due to invalid format"
        ));
    }

    Ok((result[0].to_string(), result[1].to_string()))
}

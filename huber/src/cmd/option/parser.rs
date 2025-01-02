use anyhow::anyhow;

pub fn parse_name_version(s: &str) -> anyhow::Result<(String, String)> {
    let result: Vec<_> = s.splitn(2, '@').collect();

    if result.len() != 2 {
        return Err(anyhow!(
            "Failed to parse package name version due to invalid format"
        ));
    }

    Ok((result[0].to_string(), result[1].to_string()))
}

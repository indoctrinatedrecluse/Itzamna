use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct UpdateRelease {
    pub version: String,
    pub page_url: String,
}

#[derive(Debug, Clone)]
pub enum UpdateStatus {
    Checking,
    UpToDate,
    Available(UpdateRelease),
    Failed(String),
}

#[derive(Debug, Deserialize)]
struct GithubRelease {
    tag_name: String,
    html_url: String,
    draft: bool,
    prerelease: bool,
}

pub async fn check_for_updates() -> Result<Option<UpdateRelease>, String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(8))
        .user_agent(format!("Itzamna/{}", env!("CARGO_PKG_VERSION")))
        .build()
        .map_err(|error| error.to_string())?;
    let response = client
        .get("https://api.github.com/repos/indoctrinatedrecluse/Itzamna/releases/latest")
        .header(reqwest::header::ACCEPT, "application/vnd.github+json")
        .send()
        .await
        .map_err(|error| error.to_string())?
        .error_for_status()
        .map_err(|error| error.to_string())?;
    let release: GithubRelease = response.json().await.map_err(|error| error.to_string())?;
    if release.draft || release.prerelease {
        return Ok(None);
    }

    let latest_version_text = release.tag_name.trim_start_matches('v');
    let latest = semver::Version::parse(latest_version_text)
        .map_err(|error| format!("GitHub returned an invalid release version: {error}"))?;
    let current = semver::Version::parse(env!("CARGO_PKG_VERSION"))
        .map_err(|error| format!("Application version is invalid: {error}"))?;

    if latest > current
        && release
            .html_url
            .starts_with("https://github.com/indoctrinatedrecluse/Itzamna/releases/")
    {
        Ok(Some(UpdateRelease {
            version: latest.to_string(),
            page_url: release.html_url,
        }))
    } else {
        Ok(None)
    }
}

use reqwest::Client;

pub async fn fetch_repo_info(client: &Client, owner: &str, repo: &str) -> anyhow::Result<RepoInfo> {
    let url = format!("https://api.github.com/repos/{}/{}", owner, repo);
    let res = client.get(&url)
        .header("User-Agent", "dependency-watcher")
        .send()
        .await?
        .json::<RepoInfo>()
        .await?;
    Ok(res)
}

#[derive(serde::Deserialize)]
pub struct RepoInfo {
    pub stargazers_count: u32,
    pub forks_count: u32,
    pub open_issues_count: u32,
    // add fields as needed
}
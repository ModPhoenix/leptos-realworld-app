use anyhow::Result;

use crate::api::MultipleArticlesResponse;

pub async fn get_articles() -> Result<MultipleArticlesResponse> {
    let res = reqwest::Client::new()
        .get("https://api.realworld.io/api/articles")
        .send()
        .await?;

    let text = res.text().await?;
    let result: MultipleArticlesResponse = serde_json::from_str(&text)?;

    Ok(result)
}

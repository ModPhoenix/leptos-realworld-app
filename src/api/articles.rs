use leptos::error::Result;

use crate::api::MultipleArticlesResponse;

pub async fn get_articles() -> Result<MultipleArticlesResponse> {
    let res = reqwest::Client::new()
        .get("https://api.realworld.io/api/articles")
        .send()
        .await?
        .text()
        .await?;

    let result: MultipleArticlesResponse = serde_json::from_str(&res)?;

    Ok(result)
}

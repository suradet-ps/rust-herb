use crate::models::Herb;
use leptos::prelude::*;

#[server(GetAllHerbs, "/api")]
pub async fn get_all_herbs() -> Result<Vec<Herb>, ServerFnError> {
    use crate::models::ApiResponse;

    let api_url =
        std::env::var("GOOGLE_API_URL").expect("FATAL: GOOGLE_API_URL must be set in .env file");

    let client = reqwest::Client::new();

    let resp = client.get(&api_url).send().await?;

    if !resp.status().is_success() {
        return Err(ServerFnError::new(format!(
            "HTTP error! status: {}",
            resp.status()
        )));
    }

    let json_resp: ApiResponse<Vec<Herb>> = resp.json().await?;

    if json_resp.status == "success" {
        Ok(json_resp.data)
    } else {
        Err(ServerFnError::new(
            json_resp.message.unwrap_or("Failed to fetch herbs".into()),
        ))
    }
}

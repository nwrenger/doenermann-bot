use serde::Deserialize;
use url::Url;

const API_BASE: &str = "https://api.jikan.moe/v4";
pub const IMAGE_BASE: &str = "https://cdn.myanimelist.net/images/characters/";
pub const IMAGE_EXTENSION: &str = ".webp";

use crate::{
    db::Character,
    error::{Error, Result},
};

#[derive(Deserialize)]
pub struct RadomCharacterResponse {
    pub data: CharacterData,
}

#[derive(Deserialize)]
pub struct CharacterData {
    pub mal_id: u32,
    pub images: Images,
    pub name: String,
    pub favorites: u32,
}

#[derive(Deserialize)]
pub struct Images {
    pub webp: WebpImages,
}

#[derive(Deserialize)]
pub struct WebpImages {
    pub image_url: Url,
}

#[derive(Deserialize)]
struct JikanErrorResponse {
    #[serde(rename = "type")]
    error_type: Option<String>,
}

pub async fn get_random_character() -> Result<Character> {
    let url = format!("{API_BASE}/random/characters");
    let response = reqwest::get(url).await?;

    if !response.status().is_success() {
        let status = response.status();
        let error = response.json::<JikanErrorResponse>().await.ok();
        let error_type = error
            .and_then(|error| error.error_type)
            .unwrap_or_else(|| status.to_string());

        return if status == reqwest::StatusCode::TOO_MANY_REQUESTS
            || error_type == "RateLimitException"
        {
            Err(Error::RateLimit)
        } else {
            Err(Error::Jikan(error_type))
        };
    }

    let json = response.json::<RadomCharacterResponse>().await?;
    Ok(Character::from(json))
}

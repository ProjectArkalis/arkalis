use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Media {
    #[serde(rename = "@context")]
    pub context: Option<String>,
    #[serde(rename = "@type")]
    pub welcome_type: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub thumbnail_url: Option<String>,
    pub upload_date: Option<String>,
    pub duration: Option<String>,
    pub url: Option<String>,
    pub content_url: Option<String>,
    pub embed_url: Option<String>,
    pub author: Option<Author>,
    pub thumbnail: Option<Thumbnail>,
    pub keywords: Option<String>,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub potential_action: Option<PotentialAction>,
}

#[derive(Serialize, Deserialize)]
pub struct Author {
    #[serde(rename = "@type")]
    pub author_type: Option<String>,
    pub name: Option<String>,
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct PotentialAction {
    #[serde(rename = "@type")]
    pub potential_action_type: Option<String>,
    pub target: Option<String>,
    #[serde(rename = "startOffset-input")]
    pub start_offset_input: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Thumbnail {
    #[serde(rename = "@type")]
    pub thumbnail_type: Option<String>,
    pub url: Option<String>,
}

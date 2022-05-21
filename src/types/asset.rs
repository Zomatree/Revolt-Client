use serde::{Deserialize, Serialize};
use crate::{types::ulid::ULID, AUTUMN_URL};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AssetType {
    Attachments,
    Avatars,
    Backgrounds,
    Banners,
    Icons,
}

impl std::fmt::Display for AssetType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AssetType::Attachments => "attachments",
            AssetType::Avatars => "avatars",
            AssetType::Backgrounds => "backgrounds",
            AssetType::Banners => "banners",
            AssetType::Icons => "icons",

        }.fmt(f)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum AssetMetadata {
    File {},
    Text {},
    Audio {},
    Image {
        width: u64,
        height: u64
    },
    Video {
        width: u64,
        height: u64
    },
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Asset {
    #[serde(rename = "_id")]
    pub id: ULID,

    pub tag: AssetType,
    pub size: u64,
    pub filename: String,
    pub metadata: AssetMetadata,
    pub content_type: String
}

impl Asset {
    pub fn url(&self) -> String {
        format!("https://{}/{}/{}/{}", AUTUMN_URL, self.tag, self.id, self.filename)
    }

    pub fn width_height(&self) -> (Option<u64>, Option<u64>) {
        match self.metadata {
            AssetMetadata::File {  } => (None, None),
            AssetMetadata::Text {  } => (None, None),
            AssetMetadata::Audio {  } => (None, None),
            AssetMetadata::Image { width, height } => (Some(width), Some(height)),
            AssetMetadata::Video { width, height } => (Some(width), Some(height)),
        }
    }
}

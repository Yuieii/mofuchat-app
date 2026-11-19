use serde::{Deserialize, Serialize};

use crate::prefs::{RgbColor, Service};

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum CommentRenderer {
    Text { content: String },
    Emote { name: String, url: String },
    Mention { content: String },
    TwitchCheermote { name: String, url: String, bits: u32 }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CommentPlatform {
    Twitch,
    External,
    YouTube,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserBadge {
    pub url: String,
    pub label: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommonCommentData {
    pub id: String,
    pub timestamp: u64,
    pub name: String,
    pub display_name: String,
    pub profile_image: Option<String>,
    pub renderers: Vec<CommentRenderer>,
    pub badges: Vec<UserBadge>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(untagged, rename_all = "camelCase")]
#[non_exhaustive]
pub enum CommentData {
    Twitch {
        #[serde(flatten)]
        base: CommonCommentData,

        bits: Option<u32>,
    },

    External {
        #[serde(flatten)]
        base: CommonCommentData
    },
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub id: String,
    pub service: CommentPlatform,
    pub data: CommentData,
    pub color: RgbColor,
}

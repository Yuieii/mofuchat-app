use serde::{Serialize, Deserialize};

use crate::prefs::{RgbColor, Service};

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum CommentRenderer {
    Text { content: String },
    Emote { name: String, url: String }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CommentPlatform {
    Twitch,
    External,
    YouTube,
}

#[derive(Debug, Clone, Serialize)]
#[serde(untagged, rename_all = "camelCase")]
#[non_exhaustive]
pub enum CommentData {
    Twitch {

    },

    External {
        
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub id: String,
    pub service: CommentPlatform,
    pub color: RgbColor,
    pub data: CommentData,
    pub timestamp: u64,
    pub author_name: String,
    pub author_display_name: String,
    pub author_avatar: Option<String>,
    pub renderers: Vec<CommentRenderer>,
}
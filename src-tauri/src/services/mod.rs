use serde::{Deserialize, Serialize};

use crate::comments::{Comment, CommentPlatform};

mod twitch;

pub mod adapter;
pub mod commands;

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "event", content = "data", rename_all = "camelCase")]
pub enum ServiceEvent {
    CommentCleared,
    CommentDeleted {
        id: String,
    },
    CommentCreated(Comment),
    ServiceConnected {
        id: String,
    },
    ServiceDisconnected {
        id: String,
    },
    ServiceDeactivated {
        id: String,
    },
    RequiresSignIn {
        platform: CommentPlatform,
        code: Option<String>,
        url: String,
        expires_in: u64,
    },
}

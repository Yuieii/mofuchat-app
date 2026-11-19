use serde::{Deserialize, Serialize};

use crate::comments::Comment;

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
        id: String,
        code: Option<String>,
        url: String,
    },
}

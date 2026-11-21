use serde::{Deserialize, Serialize};

use crate::prefs::{RgbColor, Service};

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum CommentRenderer {
    Text {
        content: String,
    },
    Emote {
        name: String,
        url: String,
    },
    Mention {
        content: String,
    },
    TwitchCheermote {
        name: String,
        url: String,
        bits: u32,
    },
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

/*
export interface BaseResponse {
    id: string;
    userId: string;
    liveId: string;
    name: string;
    screenName?: string;
    isOwner: boolean;
    isSupporter?: boolean;
    displayName?: string;
    nickname?: string;
    hasGift: boolean;
    autoModerated?: boolean;
    /**
     * 自動保留（autoModerated）されていたコメントが、その後モデレーター/配信者に
     * 承認されて表示に回ったことを示す。承認コメントは元の投稿時刻を保持するため、
     * 表示側で「保留から承認」を明示し、古い場合は読み上げを抑止する。
     */
    approvedAfterAutoModeration?: boolean;
    profileImage: string;
    originalProfileImage?: string;
    badges: BaseBadge[];
    timestamp: string;
    comment: string;
    speechText?: string;
    isFirstTime?: boolean;
    isRepeater?: boolean;
    commentVisible?: boolean;
    meta?: CommentMeta;
    /**
     * このコメントが運ぶユーザー属性（`name`/`profileImage`/`badges`）が部分的にしか
     * 信頼できないことを示す印。存在すること自体が印になる。
     *
     * ここに入っている項目（`name`/`profileImage`）だけが加工前の実値。入っていない
     * 項目と `badges` は表示用に加工された値やフォールバック値の可能性があるため、
     * 実際の視聴者情報として扱ってはならない。
     * このフィールド自体が無いコメントは、従来どおり全項目を実値として扱ってよい。
     */
    originalUser?: {
        name?: string;
        profileImage?: string;
    };
    /** コメントとして流れないイベントをコメント形式で扱うときに付与される。 */
    event?: CommentEvent;
}
*/

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommonCommentData {
    pub id: String,
    pub user_id: String,
    pub timestamp: String,
    pub name: String,
    pub display_name: String,
    pub profile_image: Option<String>,
    pub badges: Vec<UserBadge>,
    pub renderers: Vec<CommentRenderer>,
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
        base: CommonCommentData,
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

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::User;

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct ChatInviteLink {
    /// The invite link. If the link was created by another chat administrator,
    /// then the second part of the link will be replaced with “…”.
    pub invite_link: String,
    /// Creator of the link
    pub creator: User,
    /// `true`, if the link is primary
    pub is_primary: bool,
    /// `true`, if the link is revoked
    pub is_revoked: bool,
    /// Point in time when the link will expire or has been
    /// expired
    #[serde(with = "crate::types::serde_opt_date_from_unix_timestamp")]
    #[serde(default = "crate::types::serde_opt_date_from_unix_timestamp::none")]
    pub expire_date: Option<DateTime<Utc>>,
    /// Maximum number of users that can be members of the chat simultaneously
    /// after joining the chat via this invite link; 1-99999
    pub member_limit: Option<u32>,
}

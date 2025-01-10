use serde::Deserialize;

use crate::api::Command;

use super::{CanisterId, MessageId, MessageIndex, StringChat, UserId};

pub enum TokenError {
    Invalid(String),
    Expired,
}

#[derive(Deserialize)]
pub struct BotCommandClaims {
    pub initiator: UserId,
    pub bot: UserId,
    pub chat: StringChat,
    pub thread_root_message_index: Option<MessageIndex>,
    pub message_id: MessageId,
    pub command: Command,
    pub bot_api_gateway: CanisterId,
}

use crate::api::Command;
use crate::jwt;
use crate::jwt::Claims;
use crate::types::{
    BotCommandClaims, MessageId, MessageIndex, StringChat, TimestampMillis, TokenError, UserId,
};
use candid::Principal;

#[derive(Debug)]
pub struct BotCommandContext {
    jwt: String,
    claims: BotCommandClaims,
}

impl BotCommandContext {
    pub fn parse(jwt: String, public_key: &str, now: TimestampMillis) -> Result<Self, TokenError> {
        let claims = jwt::verify::<Claims<BotCommandClaims>>(&jwt, public_key)
            .map_err(|error| TokenError::Invalid(error.to_string()))?;

        if claims.exp_ms() > now {
            Ok(BotCommandContext {
                jwt,
                claims: claims.into_custom(),
            })
        } else {
            Err(TokenError::Expired)
        }
    }

    pub fn jwt(&self) -> &str {
        &self.jwt
    }

    pub fn initiator(&self) -> UserId {
        self.claims.initiator
    }

    pub fn bot_id(&self) -> UserId {
        self.claims.bot
    }

    pub fn chat(&self) -> &StringChat {
        &self.claims.chat
    }

    pub fn thread_root_message_index(&self) -> Option<MessageIndex> {
        self.claims.thread_root_message_index
    }

    pub fn message_id(&self) -> MessageId {
        self.claims.message_id.clone()
    }

    pub fn command(&self) -> &Command {
        &self.claims.command
    }

    pub fn bot_api_gateway(&self) -> Principal {
        self.claims.bot_api_gateway
    }
}

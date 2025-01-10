use crate::api::{Command, Message};
use crate::runtime::Runtime;
use crate::types::{
    ActionArgs, BotAction, BotCommandClaims, BotMessageAction, MessageContent, MessageId,
    MessageIndex, StringChat, TextContent, TokenError, UserId,
};
use crate::utils::jwt;

pub struct OpenChatClient<R> {
    jwt: String,
    claims: BotCommandClaims,
    runtime: R,
}

impl<R: Runtime + Clone> OpenChatClient<R> {
    pub fn build(jwt: String, public_key: &str, runtime: R) -> Result<Self, TokenError> {
        let claims = jwt::verify::<jwt::Claims<BotCommandClaims>>(&jwt, public_key)
            .map_err(|error| TokenError::Invalid(error.to_string()))?;

        if claims.exp_ms() < runtime.now() {
            return Err(TokenError::Expired);
        }

        Ok(Self {
            jwt,
            claims: claims.into_custom(),
            runtime,
        })
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

    pub fn send_text_message(&self, text: String, finalised: bool) -> Message {
        let content = MessageContent::Text(TextContent { text });

        let action = BotAction::SendMessage(BotMessageAction {
            content: content.clone(),
            finalised,
        });

        self.execute_bot_action_fire_and_forget(action);

        Message {
            id: self.claims.message_id.clone(),
            content,
            finalised,
        }
    }

    fn execute_bot_action_fire_and_forget(&self, action: BotAction) {
        let args = ActionArgs {
            action,
            jwt: self.jwt.clone(),
        };

        self.runtime.call_canister_fire_and_forget(
            self.claims.bot_api_gateway,
            "execute_bot_action",
            (args,),
        )
    }
}

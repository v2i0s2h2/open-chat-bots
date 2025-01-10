use ic_cdk::api::call::CallResult;
use oc_bots_sdk::{
    api::{Command, Message},
    jwt,
    types::{
        ActionArgs, ActionResponse, BotAction, BotApiCallError, BotCommandClaims, BotMessageAction,
        CanisterId, MessageContent, MessageId, MessageIndex, StringChat, TextContent, TokenError,
        UserId,
    },
};

use super::env;

pub struct OpenChatClient {
    jwt: String,
    claims: BotCommandClaims,
}

impl OpenChatClient {
    pub fn build(jwt: String, public_key: &str) -> Result<Self, TokenError> {
        let claims = jwt::verify::<jwt::Claims<BotCommandClaims>>(&jwt, public_key)
            .map_err(|error| TokenError::Invalid(error.to_string()))?;

        if claims.exp_ms() < env::now() {
            return Err(TokenError::Expired);
        }

        Ok(Self {
            jwt,
            claims: claims.into_custom(),
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

        self.execute_bot_action(action);

        Message {
            id: self.claims.message_id.clone(),
            content,
            finalised,
        }
    }

    fn execute_bot_action(&self, action: BotAction) {
        let args = ActionArgs {
            action,
            jwt: self.jwt.clone(),
        };

        ic_cdk::spawn(execute_bot_action_inner(self.claims.bot_api_gateway, args));

        async fn execute_bot_action_inner(bot_api_gateway: CanisterId, args: ActionArgs) {
            let response: CallResult<(ActionResponse,)> =
                ic_cdk::call(bot_api_gateway, "execute_bot_action", (&args,)).await;

            let result = match response.map(|r| r.0) {
                Ok(resp) => resp,
                Err((code, message)) => Err(BotApiCallError::C2CError(code as i32, message)),
            };

            if let Some(error) = result.err() {
                ic_cdk::println!("Failed to execute bot action: {:?}", error);
            }
        }
    }
}

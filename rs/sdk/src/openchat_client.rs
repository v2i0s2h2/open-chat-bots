use crate::api::Message;
use crate::runtime::Runtime;
use crate::types::{
    ActionArgs, ActionResponse, BotAction, BotCommandContext, BotMessageAction, MessageContent,
    TextContent,
};

pub struct OpenChatClient<R> {
    runtime: R,
}

impl<R: Runtime> OpenChatClient<R> {
    pub const fn new(runtime: R) -> Self {
        Self { runtime }
    }

    pub fn send_text_message<F: FnOnce(ActionArgs, ActionResponse) + 'static>(
        &self,
        context: &BotCommandContext,
        text: String,
        finalised: bool,
        on_response: F,
    ) -> Message {
        let content = MessageContent::Text(TextContent { text });

        let action = BotAction::SendMessage(BotMessageAction {
            content: content.clone(),
            finalised,
        });

        self.execute_bot_action_fire_and_forget(context, action, on_response);

        Message {
            id: context.message_id(),
            content,
            finalised,
        }
    }

    fn execute_bot_action_fire_and_forget<F: FnOnce(ActionArgs, ActionResponse) + 'static>(
        &self,
        context: &BotCommandContext,
        action: BotAction,
        on_result: F,
    ) {
        let args = ActionArgs {
            action,
            jwt: context.jwt().to_string(),
        };

        self.runtime.call_canister_fire_and_forget(
            context.bot_api_gateway(),
            "execute_bot_action",
            (args.clone(),),
            move |result| on_result(args, result),
        )
    }
}

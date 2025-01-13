use crate::state;
use oc_bots_sdk::api::{InternalError, SuccessResult};
use oc_bots_sdk::types::BotCommandContext;
use oc_bots_sdk_canister::OPENCHAT_CLIENT;

pub fn greet(context: BotCommandContext) -> Result<SuccessResult, InternalError> {
    let user_id = context.initiator();
    let text = format!("hello @UserId({user_id})");

    // Send the message to OpenChat but don't wait for the response
    let message =
        OPENCHAT_CLIENT.send_text_message(&context, text, true, |args, response| match response {
            Ok(result) if result.0.is_ok() => {
                state::mutate(|state| state.increment_jokes_sent());
            }
            error => {
                ic_cdk::println!("greet send_text_message: {args:?}, {error:?}");
            }
        });

    Ok(SuccessResult {
        message: Some(message),
    })
}

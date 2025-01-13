use crate::state;
use oc_bots_sdk::api::{InternalError, SuccessResult};
use oc_bots_sdk::types::BotCommandContext;
use oc_bots_sdk_canister::OPENCHAT_CLIENT;

pub fn joke(context: BotCommandContext) -> Result<SuccessResult, InternalError> {
    let text = state::read(|state| state.get_random_joke());

    // Send the message to OpenChat but don't wait for the response
    let message =
        OPENCHAT_CLIENT.send_text_message(&context, text, true, |args, response| match response {
            Ok(result) if result.0.is_ok() => {
                state::mutate(|state| state.increment_jokes_sent());
            }
            error => {
                ic_cdk::println!("joke send_text_message: {args:?}, {error:?}");
            }
        });

    Ok(SuccessResult {
        message: Some(message),
    })
}

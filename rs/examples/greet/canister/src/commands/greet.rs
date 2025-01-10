use oc_bots_sdk::api::{InternalError, SuccessResult};
use oc_bots_sdk::OpenChatClient;
use oc_bots_sdk_canister::CanisterRuntime;

use crate::state;

pub fn greet(client: OpenChatClient<CanisterRuntime>) -> Result<SuccessResult, InternalError> {
    let user_id = client.initiator();
    let text = format!("hello @UserId({user_id})");

    // Send the message to OpenChat but don't wait for the response
    let message = client.send_text_message(text, true);

    state::mutate(|state| state.increment_greets_sent());

    Ok(SuccessResult {
        message: Some(message),
    })
}

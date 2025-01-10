use oc_bots_sdk::api::{InternalError, SuccessResult};
use oc_bots_sdk_canister::OpenChatClient;

use crate::state;

pub fn joke(client: OpenChatClient) -> Result<SuccessResult, InternalError> {
    let text = state::read(|state| state.get_random_joke());

    // Send the message to OpenChat but don't wait for the response
    let message = client.send_text_message(text, true);

    state::mutate(|state| state.increment_jokes_sent());

    Ok(SuccessResult {
        message: Some(message),
    })
}

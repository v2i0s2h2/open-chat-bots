use oc_bots_sdk::{
    api::{BadRequest, CommandResponse},
    types::TokenError,
};
use oc_bots_sdk_canister::OpenChatClient;

use crate::{
    commands,
    state::{self},
};

pub async fn execute_command(jwt: &str) -> CommandResponse {
    let public_key = state::read(|state| state.oc_public_key().to_string());

    let client = match OpenChatClient::build(jwt.to_string(), &public_key) {
        Ok(a) => a,
        Err(bad_request) => match bad_request {
            TokenError::Invalid(_) => {
                return CommandResponse::BadRequest(BadRequest::AccessTokenInvalid)
            }
            TokenError::Expired => {
                return CommandResponse::BadRequest(BadRequest::AccessTokenExpired)
            }
        },
    };

    let result = match client.command().name.as_str() {
        "greet" => commands::greet(client),
        "joke" => commands::joke(client),
        _ => return CommandResponse::BadRequest(BadRequest::CommandNotFound),
    };

    match result {
        Ok(success) => CommandResponse::Success(success),
        Err(internal_error) => CommandResponse::InternalError(internal_error),
    }
}

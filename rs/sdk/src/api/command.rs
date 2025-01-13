use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::types::{MessageContent, MessageId, UserId};

#[derive(Serialize, Deserialize, Debug)]
pub struct Command {
    pub name: String,
    pub args: Vec<CommandArg>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandArg {
    pub name: String,
    pub value: CommandArgValue,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum CommandArgValue {
    String(String),
    Number(f64),
    Boolean(bool),
    User(UserId),
}

#[allow(clippy::large_enum_variant)]
#[derive(Serialize)]
pub enum CommandResponse {
    Success(SuccessResult),
    BadRequest(BadRequest),
    InternalError(InternalError),
}

#[derive(Serialize)]
pub struct SuccessResult {
    pub message: Option<Message>,
}

#[derive(Serialize)]
pub struct Message {
    pub id: MessageId,
    pub content: MessageContent,
    pub finalised: bool,
}

#[derive(Serialize)]
pub enum BadRequest {
    AccessTokenNotFound,
    AccessTokenInvalid,
    AccessTokenExpired,
    CommandNotFound,
    ArgsInvalid,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum InternalError {
    Invalid(String),
    CanisterError(CanisterError),
    C2CError(i32, String),
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum CanisterError {
    NotAuthorized,
    Frozen,
    Other(String),
}

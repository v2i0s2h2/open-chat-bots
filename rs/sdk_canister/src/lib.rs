mod canister_runtime;
pub mod env;

pub use canister_runtime::CanisterRuntime;
use oc_bots_sdk::OpenChatClient;

pub const OPENCHAT_CLIENT: OpenChatClient<CanisterRuntime> = OpenChatClient::new(CanisterRuntime);

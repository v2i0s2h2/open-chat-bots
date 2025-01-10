use crate::types::{CanisterId, TimestampMillis};
use candid::utils::{ArgumentDecoder, ArgumentEncoder};
use std::future::Future;

pub trait Runtime {
    fn call_canister<A: ArgumentEncoder, R: for<'a> ArgumentDecoder<'a>>(
        &self,
        canister_id: CanisterId,
        method_name: &str,
        args: A,
    ) -> impl Future<Output = Result<R, (i32, String)>>;

    fn call_canister_fire_and_forget<A: ArgumentEncoder + 'static>(
        &self,
        canister_id: CanisterId,
        method_name: &'static str,
        args: A,
    );

    fn now(&self) -> TimestampMillis;
}

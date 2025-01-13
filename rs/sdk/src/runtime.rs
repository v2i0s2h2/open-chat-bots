use crate::types::{CallResult, CanisterId, TimestampMillis};
use candid::utils::{ArgumentDecoder, ArgumentEncoder};
use std::future::Future;

pub trait Runtime {
    fn call_canister<A: ArgumentEncoder, R: for<'a> ArgumentDecoder<'a>>(
        &self,
        canister_id: CanisterId,
        method_name: &str,
        args: A,
    ) -> impl Future<Output = CallResult<R>>;

    fn call_canister_fire_and_forget<
        A: ArgumentEncoder + 'static,
        R: for<'a> ArgumentDecoder<'a>,
        F: FnOnce(CallResult<R>) + 'static,
    >(
        &self,
        canister_id: CanisterId,
        method_name: &'static str,
        args: A,
        on_result: F,
    );

    fn now(&self) -> TimestampMillis;
}

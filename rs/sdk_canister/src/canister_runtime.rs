use candid::utils::{ArgumentDecoder, ArgumentEncoder};
use oc_bots_sdk::types::{CallResult, CanisterId, TimestampMillis};
use oc_bots_sdk::Runtime;

#[derive(Clone, Default)]
pub struct CanisterRuntime;

impl Runtime for CanisterRuntime {
    async fn call_canister<A: ArgumentEncoder, R: for<'a> ArgumentDecoder<'a>>(
        &self,
        canister_id: CanisterId,
        method_name: &str,
        args: A,
    ) -> CallResult<R> {
        call_canister_inner(canister_id, method_name, args).await
    }

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
    ) {
        ic_cdk::spawn(async move {
            let result = call_canister_inner::<A, R>(canister_id, method_name, args).await;
            on_result(result);
        });
    }

    fn now(&self) -> TimestampMillis {
        crate::env::now()
    }
}

async fn call_canister_inner<A: ArgumentEncoder, R: for<'a> ArgumentDecoder<'a>>(
    canister_id: CanisterId,
    method_name: &str,
    args: A,
) -> CallResult<R> {
    match ic_cdk::api::call::call(canister_id, method_name, args).await {
        Ok(result) => Ok(result),
        Err((code, msg)) => Err((code as i32, msg)),
    }
}

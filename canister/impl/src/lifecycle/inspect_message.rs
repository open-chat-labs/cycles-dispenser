use ic_cdk_macros::inspect_message;

#[inspect_message]
fn inspect_message() {
    // By not calling `ic_cdk::api::call::accept_message()` we reject all ingress messages
}

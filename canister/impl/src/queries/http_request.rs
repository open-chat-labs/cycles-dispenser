use crate::{read_state, State};
use ic_cdk_macros::query;
use ic_ledger_types::{AccountIdentifier, DEFAULT_SUBACCOUNT};
use serde::Serialize;
use serde_bytes::ByteBuf;
use std::borrow::Cow;
use types::{HeaderField, HttpRequest, HttpResponse};

#[query]
fn http_request(request: HttpRequest) -> HttpResponse {
    let path = request.url.trim_matches('/').to_lowercase();

    match path.as_str() {
        "metrics" => read_state(|state| to_json_response(&state.metrics())),
        "ledger_account" => read_state(get_ledger_account_impl),
        _ => HttpResponse::not_found(),
    }
}

fn to_json_response<T: Serialize>(data: &T) -> HttpResponse {
    let body = serde_json::to_string(data).unwrap().into_bytes();

    HttpResponse {
        status_code: 200,
        headers: vec![
            HeaderField("Content-Type".to_string(), "application/json".to_string()),
            HeaderField("Content-Length".to_string(), body.len().to_string()),
        ],
        body: Cow::Owned(ByteBuf::from(body)),
    }
}

fn get_ledger_account_impl(state: &State) -> HttpResponse {
    let ledger_account =
        AccountIdentifier::new(&state.env.canister_id(), &DEFAULT_SUBACCOUNT).to_string();

    let body = ledger_account.into_bytes();

    HttpResponse {
        status_code: 200,
        headers: vec![
            HeaderField("Content-Type".to_string(), "text/plain".to_string()),
            HeaderField("Content-Length".to_string(), body.len().to_string()),
        ],
        body: Cow::Owned(ByteBuf::from(body)),
    }
}

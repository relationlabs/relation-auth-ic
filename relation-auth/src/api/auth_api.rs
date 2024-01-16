use ic_cdk::api;
use ic_cdk::export::candid::{candid_method, export_service};
use ic_cdk_macros::*;

use crate::config;
use crate::core::auth::verify_sign;
use crate::core::jwt::jwt_sign;
use crate::model::auth::AuthRequest;
use crate::model::auth::{RelationClaims, Subject};
use crate::ApiResult;

const VERIFY_SIGN_FAIL: &str = "verify sign fail";
const SIGN_JWT_FAIL: &str = "sign jwt fail";

#[query]
#[candid_method(query)]
fn cycles_balance() -> u64 {
    api::canister_balance()
}

#[query]
#[candid_method(query)]
fn auth(req: AuthRequest) -> ApiResult<String> {
    verify_sign(&req)
        .map(|address| {
            let subject = Subject {
                account_source: req.chain_name,
                address,
                user_principal: String::default(),
            };
            let claims = RelationClaims {
                subject: serde_json::to_string(&subject).expect("Invalid subject"),
            };
            jwt_sign(claims, config::TOKEN_EXPIRATION_SECS_DEFAULT)
                .map(|token| Ok(token))
                .unwrap_or(Err(SIGN_JWT_FAIL.into()))
        })
        .unwrap_or(Err(VERIFY_SIGN_FAIL.into()))
}

#[query]
#[candid_method(query)]
fn __get_candid_interface_tmp_hack() -> String {
    __export_service()
}

export_service!();

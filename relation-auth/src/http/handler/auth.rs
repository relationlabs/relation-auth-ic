//! HTTP JWT AUTH

use serde_bytes::ByteBuf;

use crate::config;
use crate::core::auth as auth_core;
use crate::core::jwt;
use crate::core::jwt::{jwt_sign, jwt_verify};
use crate::http::{HttpRequest, HttpResponse};
use crate::model::auth::{AuthRequest, AuthResponse, AuthToken, RelationClaims, Subject};

/// Generate JWT token
pub fn jwt_generate_token(req: HttpRequest) -> HttpResponse {
    let auth_req: Result<AuthRequest, _> = req.body();

    match auth_req {
        Ok(req) => auth_core::verify_sign(&req)
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
                    .map(|token| HttpResponse::ok(Some(AuthResponse::ok(&token, "success"))))
                    .unwrap_or(HttpResponse::bad_request())
            })
            .unwrap_or(HttpResponse::unauthorized()),
        Err(_err) => HttpResponse::bad_request(),
    }
}

/// Verify JWT token
pub fn jwt_verify_token(req: HttpRequest) -> HttpResponse {
    let auth_token: Result<AuthToken, _> = req.body();
    match auth_token {
        Ok(token) => jwt_verify(&token.token)
            .map(|claims| HttpResponse::ok(Some(claims)))
            .unwrap_or(HttpResponse::unauthorized()),
        _ => HttpResponse::unauthorized(),
    }
}

pub fn jwk() -> HttpResponse {
    HttpResponse::new(200, Some(ByteBuf::from(jwt::jwk().as_bytes())), false)
}

pub fn pem() -> HttpResponse {
    HttpResponse::new(200, Some(ByteBuf::from(jwt::pem().as_bytes())), false)
}

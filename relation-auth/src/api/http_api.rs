//! HTTP API for query and update

use ic_cdk_macros::*;

use crate::http::{handler, HttpRequest, HttpResponse};

/// Handle HTTP request(only accept JSON)
#[query]
pub fn http_request(req: HttpRequest) -> HttpResponse {
    // MAY useful for debug
    // ic_cdk::println!("request: {:?}", &req);
    // ic_cdk::println!("query_params: {:?}", req.query_params());

    // only accept JSON
    if let "/jwk" = req.path() {
        return handler::auth::jwk();
    }
    if let "/pem" = req.path() {
        return handler::auth::pem();
    }
    if !req.is_content_type_json() {
        return HttpResponse::unsupported_media_type();
    }
    handle_http_query(req)
}

fn handle_http_query(req: HttpRequest) -> HttpResponse {
    match req.path() {
        "/auth" => handler::auth::jwt_generate_token(req),
        "/jwt/verify" => handler::auth::jwt_verify_token(req),
        _ => HttpResponse::forbidden(),
    }
}

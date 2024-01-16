use chrono::Duration;
use nano_jwt::{algo::ES256, Jwt, JwtClaims, TimeOptions};
use once_cell::sync::OnceCell;

use crate::model::auth::RelationClaims;
use crate::{util, ApiResult};

pub fn jwt_sign(claims: RelationClaims, expiration_secs: u64) -> ApiResult<String> {
    let time_options = TimeOptions::new(Duration::seconds(15), util::utc_time_now);
    let claims = JwtClaims::new(claims)
        .with_expiration_and_issuance(&time_options, Duration::seconds(expiration_secs as i64));
    jwt().sign(&claims).map_err(|e| e.to_string())
}

pub fn jwt_verify(token: &str) -> ApiResult<RelationClaims> {
    let time_options = TimeOptions::new(Duration::seconds(15), util::utc_time_now);

    jwt()
        .verify::<RelationClaims, _>(token, &time_options)
        .map(|verified| verified.claims.custom)
        .map_err(|e| e.to_string())
}

static JWT: OnceCell<Jwt<ES256>> = OnceCell::new();

fn jwt() -> &'static Jwt<ES256> {
    JWT.get_or_init(|| {
        let jwk = include_str!("../../data/es256-jwk-private.json");
        Jwt::new(ES256::from_jwk(jwk))
    })
}

static JWK: OnceCell<String> = OnceCell::new();

pub fn jwk() -> &'static String {
    JWK.get_or_init(|| {
        let jwk = include_str!("../../data/es256-jwk-public.json");
        jwk.to_string()
    })
}

pub fn pem() -> &'static String {
    JWK.get_or_init(|| {
        let jwk = include_str!("../../data/es256-public.pem");
        jwk.to_string()
    })
}

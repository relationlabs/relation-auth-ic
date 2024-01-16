use ic_cdk::export::{candid::CandidType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationClaims {
    #[serde(rename = "sub")]
    pub subject: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subject {
    #[serde(rename = "accountSource")]
    pub account_source: String,

    #[serde(rename = "address")]
    pub address: String,

    #[serde(rename = "userPrincipal")]
    pub user_principal: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthToken {
    pub token: String,
}

#[derive(Debug, CandidType, Clone, Deserialize)]
pub struct AuthRequest {
    pub pk: String,
    pub message: String,
    pub decoded_signature: String,
    pub algorithm: Algorithm,
    pub wallet_name: String,
    pub chain_name: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct AuthResponse {
    /// success or other kinds of error codes
    pub code: String,
    /// token will be none for failure response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

impl AuthResponse {
    pub fn ok(token: &str, code: &str) -> Self {
        Self {
            token: Some(token.to_string()),
            code: code.to_string(),
        }
    }
    pub fn error(code: &str) -> Self {
        Self {
            token: None,
            code: code.to_string(),
        }
    }
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum Algorithm {
    #[serde(rename = "secp256k1")]
    Secp256k1,
    #[serde(rename = "sr25519")]
    Sr25519,
    #[serde(rename = "ed25519")]
    Ed25519,
    #[serde(rename = "none")]
    None,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum ChainName {
    #[serde(rename = "ic")]
    IC,
    #[serde(rename = "eth")]
    ETH,
    #[serde(rename = "polkadot")]
    Polkadot,
    #[serde(rename = "solana")]
    Solana,
}

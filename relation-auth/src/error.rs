use ic_cdk::export::candid;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WalletError {
    #[error("IC save error: {0}")]
    ICSave(#[from] candid::Error),

    #[error("IC restore error: {0}")]
    ICRestore(String),

    #[error("secp256k1 error")]
    Secp256k1(#[from] libsecp256k1::Error),

    #[error("invalid hex")]
    InvalidHex(#[from] hex::FromHexError),

    #[error("invalid wallet type")]
    InvalidWalletType(#[from] strum::ParseError),

    #[error("wallet address not found: {0}")]
    WalletNotFound(String),
}

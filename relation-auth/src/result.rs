use crate::WalletError;

/// Wallet result
pub type WalletResult<T = ()> = std::result::Result<T, WalletError>;
/// IC API result (error must be String)
pub type ApiError = String;
pub type ApiResult<T = ()> = std::result::Result<T, ApiError>;

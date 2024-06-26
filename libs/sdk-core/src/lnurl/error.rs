use std::{array::TryFromSliceError, string::FromUtf8Error};

use crate::bitcoin::{bech32, secp256k1, util::bip32};
use crate::{invoice::InvoiceError, node_api::NodeError};

pub type LnUrlResult<T, E = LnUrlError> = Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum LnUrlError {
    #[error("{0}")]
    Generic(String),

    #[error(transparent)]
    InvalidInvoice(#[from] InvoiceError),

    #[error("{0}")]
    InvalidUri(String),

    #[error("{0}")]
    ServiceConnectivity(String),
}

impl LnUrlError {
    pub(crate) fn generic(err: &str) -> Self {
        Self::Generic(err.to_string())
    }

    pub(crate) fn invalid_uri(err: &str) -> Self {
        Self::InvalidUri(err.to_string())
    }
}

impl From<aes::cipher::InvalidLength> for LnUrlError {
    fn from(err: aes::cipher::InvalidLength) -> Self {
        Self::Generic(err.to_string())
    }
}

impl From<aes::cipher::block_padding::UnpadError> for LnUrlError {
    fn from(err: aes::cipher::block_padding::UnpadError) -> Self {
        Self::Generic(err.to_string())
    }
}

impl From<base64::DecodeError> for LnUrlError {
    fn from(err: base64::DecodeError) -> Self {
        Self::Generic(err.to_string())
    }
}

impl From<bip32::Error> for LnUrlError {
    fn from(err: bip32::Error) -> Self {
        Self::Generic(err.to_string())
    }
}

impl From<bech32::Error> for LnUrlError {
    fn from(err: bech32::Error) -> Self {
        Self::Generic(err.to_string())
    }
}

impl From<FromUtf8Error> for LnUrlError {
    fn from(err: FromUtf8Error) -> Self {
        Self::Generic(err.to_string())
    }
}

impl From<NodeError> for LnUrlError {
    fn from(value: NodeError) -> Self {
        match value {
            NodeError::InvalidInvoice(err) => Self::InvalidInvoice(err),
            NodeError::ServiceConnectivity(err) => Self::ServiceConnectivity(err),
            _ => Self::Generic(value.to_string()),
        }
    }
}

impl From<secp256k1::Error> for LnUrlError {
    fn from(err: secp256k1::Error) -> Self {
        Self::Generic(err.to_string())
    }
}

impl From<serde_json::Error> for LnUrlError {
    fn from(err: serde_json::Error) -> Self {
        Self::ServiceConnectivity(err.to_string())
    }
}

impl From<TryFromSliceError> for LnUrlError {
    fn from(err: TryFromSliceError) -> Self {
        Self::Generic(err.to_string())
    }
}

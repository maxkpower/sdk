/*
 * Bitwarden Internal API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: latest
 *
 * Generated by: https://openapi-generator.tech
 */

use serde::{Deserialize, Serialize};

use crate::models;

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AuthenticatorTransport {
    #[serde(rename = "usb")]
    Usb,
    #[serde(rename = "nfc")]
    Nfc,
    #[serde(rename = "ble")]
    Ble,
    #[serde(rename = "internal")]
    Internal,
}

impl std::fmt::Display for AuthenticatorTransport {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Usb => write!(f, "usb"),
            Self::Nfc => write!(f, "nfc"),
            Self::Ble => write!(f, "ble"),
            Self::Internal => write!(f, "internal"),
        }
    }
}

impl Default for AuthenticatorTransport {
    fn default() -> AuthenticatorTransport {
        Self::Usb
    }
}

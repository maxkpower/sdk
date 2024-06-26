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
#[repr(i64)]
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    serde_repr::Serialize_repr,
    serde_repr::Deserialize_repr,
)]
pub enum PaymentMethodType {
    Card = 0,
    BankAccount = 1,
    PayPal = 2,
    BitPay = 3,
    Credit = 4,
    WireTransfer = 5,
    Check = 8,
    None = 255,
}

impl std::fmt::Display for PaymentMethodType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Card => write!(f, "0"),
            Self::BankAccount => write!(f, "1"),
            Self::PayPal => write!(f, "2"),
            Self::BitPay => write!(f, "3"),
            Self::Credit => write!(f, "4"),
            Self::WireTransfer => write!(f, "5"),
            Self::Check => write!(f, "8"),
            Self::None => write!(f, "255"),
        }
    }
}

impl Default for PaymentMethodType {
    fn default() -> PaymentMethodType {
        Self::Card
    }
}

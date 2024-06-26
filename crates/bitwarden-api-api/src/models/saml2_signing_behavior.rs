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
pub enum Saml2SigningBehavior {
    IfIdpWantAuthnRequestsSigned = 0,
    Always = 1,
    Never = 3,
}

impl std::fmt::Display for Saml2SigningBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::IfIdpWantAuthnRequestsSigned => write!(f, "0"),
            Self::Always => write!(f, "1"),
            Self::Never => write!(f, "3"),
        }
    }
}

impl Default for Saml2SigningBehavior {
    fn default() -> Saml2SigningBehavior {
        Self::IfIdpWantAuthnRequestsSigned
    }
}

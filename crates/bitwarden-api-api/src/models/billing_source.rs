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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BillingSource {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<models::PaymentMethodType>,
    #[serde(rename = "cardBrand", skip_serializing_if = "Option::is_none")]
    pub card_brand: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "needsVerification", skip_serializing_if = "Option::is_none")]
    pub needs_verification: Option<bool>,
}

impl BillingSource {
    pub fn new() -> BillingSource {
        BillingSource {
            r#type: None,
            card_brand: None,
            description: None,
            needs_verification: None,
        }
    }
}

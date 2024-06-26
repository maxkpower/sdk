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
pub struct BillingCustomerDiscount {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "percentOff", skip_serializing_if = "Option::is_none")]
    pub percent_off: Option<f64>,
    #[serde(rename = "appliesTo", skip_serializing_if = "Option::is_none")]
    pub applies_to: Option<Vec<String>>,
}

impl BillingCustomerDiscount {
    pub fn new() -> BillingCustomerDiscount {
        BillingCustomerDiscount {
            id: None,
            active: None,
            percent_off: None,
            applies_to: None,
        }
    }
}

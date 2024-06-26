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
pub struct CipherCardModel {
    #[serde(rename = "cardholderName", skip_serializing_if = "Option::is_none")]
    pub cardholder_name: Option<String>,
    #[serde(rename = "brand", skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
    #[serde(rename = "number", skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[serde(rename = "expMonth", skip_serializing_if = "Option::is_none")]
    pub exp_month: Option<String>,
    #[serde(rename = "expYear", skip_serializing_if = "Option::is_none")]
    pub exp_year: Option<String>,
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

impl CipherCardModel {
    pub fn new() -> CipherCardModel {
        CipherCardModel {
            cardholder_name: None,
            brand: None,
            number: None,
            exp_month: None,
            exp_year: None,
            code: None,
        }
    }
}

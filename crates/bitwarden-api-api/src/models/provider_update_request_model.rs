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
pub struct ProviderUpdateRequestModel {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "businessName", skip_serializing_if = "Option::is_none")]
    pub business_name: Option<String>,
    #[serde(rename = "billingEmail")]
    pub billing_email: String,
}

impl ProviderUpdateRequestModel {
    pub fn new(name: String, billing_email: String) -> ProviderUpdateRequestModel {
        ProviderUpdateRequestModel {
            name,
            business_name: None,
            billing_email,
        }
    }
}

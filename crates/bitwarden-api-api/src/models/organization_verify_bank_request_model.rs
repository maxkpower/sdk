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
pub struct OrganizationVerifyBankRequestModel {
    #[serde(rename = "amount1")]
    pub amount1: i32,
    #[serde(rename = "amount2")]
    pub amount2: i32,
}

impl OrganizationVerifyBankRequestModel {
    pub fn new(amount1: i32, amount2: i32) -> OrganizationVerifyBankRequestModel {
        OrganizationVerifyBankRequestModel { amount1, amount2 }
    }
}

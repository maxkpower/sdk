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
pub struct ProviderUserBulkConfirmRequestModelEntry {
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "key")]
    pub key: String,
}

impl ProviderUserBulkConfirmRequestModelEntry {
    pub fn new(id: uuid::Uuid, key: String) -> ProviderUserBulkConfirmRequestModelEntry {
        ProviderUserBulkConfirmRequestModelEntry { id, key }
    }
}

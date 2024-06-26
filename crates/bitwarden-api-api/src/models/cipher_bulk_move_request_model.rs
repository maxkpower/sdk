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
pub struct CipherBulkMoveRequestModel {
    #[serde(rename = "ids")]
    pub ids: Vec<String>,
    #[serde(rename = "folderId", skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<String>,
}

impl CipherBulkMoveRequestModel {
    pub fn new(ids: Vec<String>) -> CipherBulkMoveRequestModel {
        CipherBulkMoveRequestModel {
            ids,
            folder_id: None,
        }
    }
}

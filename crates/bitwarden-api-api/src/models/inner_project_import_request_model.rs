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
pub struct InnerProjectImportRequestModel {
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "name")]
    pub name: String,
}

impl InnerProjectImportRequestModel {
    pub fn new(id: uuid::Uuid, name: String) -> InnerProjectImportRequestModel {
        InnerProjectImportRequestModel { id, name }
    }
}

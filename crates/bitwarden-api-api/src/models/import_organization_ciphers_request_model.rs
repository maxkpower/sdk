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
pub struct ImportOrganizationCiphersRequestModel {
    #[serde(rename = "collections", skip_serializing_if = "Option::is_none")]
    pub collections: Option<Vec<models::CollectionWithIdRequestModel>>,
    #[serde(rename = "ciphers", skip_serializing_if = "Option::is_none")]
    pub ciphers: Option<Vec<models::CipherRequestModel>>,
    #[serde(
        rename = "collectionRelationships",
        skip_serializing_if = "Option::is_none"
    )]
    pub collection_relationships: Option<Vec<models::Int32Int32KeyValuePair>>,
}

impl ImportOrganizationCiphersRequestModel {
    pub fn new() -> ImportOrganizationCiphersRequestModel {
        ImportOrganizationCiphersRequestModel {
            collections: None,
            ciphers: None,
            collection_relationships: None,
        }
    }
}

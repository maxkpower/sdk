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
pub struct OrganizationSsoRequestModel {
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[serde(rename = "identifier", skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(rename = "data")]
    pub data: Box<models::SsoConfigurationDataRequest>,
}

impl OrganizationSsoRequestModel {
    pub fn new(
        enabled: bool,
        data: models::SsoConfigurationDataRequest,
    ) -> OrganizationSsoRequestModel {
        OrganizationSsoRequestModel {
            enabled,
            identifier: None,
            data: Box::new(data),
        }
    }
}

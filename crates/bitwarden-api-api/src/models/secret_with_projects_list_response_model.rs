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
pub struct SecretWithProjectsListResponseModel {
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    #[serde(rename = "secrets", skip_serializing_if = "Option::is_none")]
    pub secrets: Option<Vec<models::SecretsWithProjectsInnerSecret>>,
    #[serde(rename = "projects", skip_serializing_if = "Option::is_none")]
    pub projects: Option<Vec<models::SecretWithProjectsInnerProject>>,
}

impl SecretWithProjectsListResponseModel {
    pub fn new() -> SecretWithProjectsListResponseModel {
        SecretWithProjectsListResponseModel {
            object: None,
            secrets: None,
            projects: None,
        }
    }
}

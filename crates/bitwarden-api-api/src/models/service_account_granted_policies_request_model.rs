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
pub struct ServiceAccountGrantedPoliciesRequestModel {
    #[serde(
        rename = "projectGrantedPolicyRequests",
        skip_serializing_if = "Option::is_none"
    )]
    pub project_granted_policy_requests: Option<Vec<models::GrantedAccessPolicyRequest>>,
}

impl ServiceAccountGrantedPoliciesRequestModel {
    pub fn new() -> ServiceAccountGrantedPoliciesRequestModel {
        ServiceAccountGrantedPoliciesRequestModel {
            project_granted_policy_requests: None,
        }
    }
}

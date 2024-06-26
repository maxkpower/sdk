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
pub struct OrganizationSponsorshipSyncResponseModel {
    #[serde(rename = "sponsorshipsBatch", skip_serializing_if = "Option::is_none")]
    pub sponsorships_batch: Option<Vec<models::OrganizationSponsorshipResponseModel>>,
}

impl OrganizationSponsorshipSyncResponseModel {
    pub fn new() -> OrganizationSponsorshipSyncResponseModel {
        OrganizationSponsorshipSyncResponseModel {
            sponsorships_batch: None,
        }
    }
}

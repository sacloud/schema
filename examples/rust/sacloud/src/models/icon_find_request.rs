/*
 * SAKURA Cloud APIs
 *
 * This is a definitions for SAKURA Cloud APIs.
 *
 * The version of the OpenAPI document: 0.0.1
 * Contact: sacloud.users@gmail.com
 * Generated by: https://openapi-generator.tech
 */

use std::fmt;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IconFindRequest {
    #[serde(rename = "Count", skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "Exclude", skip_serializing_if = "Option::is_none")]
    pub exclude: Option<Vec<String>>,
    #[serde(rename = "From", skip_serializing_if = "Option::is_none")]
    pub from: Option<i32>,
    #[serde(rename = "Include", skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
    #[serde(rename = "Sort", skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<String>>,
    #[serde(rename = "Filter", skip_serializing_if = "Option::is_none")]
    pub filter: Option<crate::models::IconFindFilter>,
}

impl IconFindRequest {
    pub fn new() -> IconFindRequest {
        IconFindRequest {
            count: None,
            exclude: None,
            from: None,
            include: None,
            sort: None,
            filter: None,
        }
    }
}

impl fmt::Display for IconFindRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let serialized = serde_json::to_string(&self).unwrap();
        write!(f, "{}", serialized)
    }
}
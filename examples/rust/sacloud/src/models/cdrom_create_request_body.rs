/*
 * SAKURA Cloud APIs
 *
 * This is a definitions for SAKURA Cloud APIs.
 *
 * The version of the OpenAPI document: 0.0.1
 * Contact: sacloud.users@gmail.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CdromCreateRequestBody {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "SizeMB")]
    pub size_mb: crate::models::CdromSizes,
    #[serde(rename = "Icon", skip_serializing_if = "Option::is_none")]
    pub icon: Option<crate::models::IdEnvelope>,
}

impl CdromCreateRequestBody {
    pub fn new(name: String, size_mb: crate::models::CdromSizes) -> CdromCreateRequestBody {
        CdromCreateRequestBody {
            name,
            description: None,
            tags: None,
            size_mb,
            icon: None,
        }
    }
}


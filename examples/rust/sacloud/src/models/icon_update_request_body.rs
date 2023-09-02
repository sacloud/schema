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
pub struct IconUpdateRequestBody {
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

impl IconUpdateRequestBody {
    pub fn new() -> IconUpdateRequestBody {
        IconUpdateRequestBody {
            name: None,
            tags: None,
        }
    }
}


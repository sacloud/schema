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
pub struct IconUpdateRequest {
    #[serde(rename = "Icon")]
    pub icon: crate::models::IconUpdateRequestBody,
}

impl IconUpdateRequest {
    pub fn new(icon: crate::models::IconUpdateRequestBody) -> IconUpdateRequest {
        IconUpdateRequest {
            icon,
        }
    }
}



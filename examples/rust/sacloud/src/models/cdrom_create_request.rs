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
pub struct CdromCreateRequest {
    #[serde(rename = "CDROM")]
    pub CDROM: crate::models::CdromCreateRequestBody,
}

impl CdromCreateRequest {
    pub fn new(CDROM: crate::models::CdromCreateRequestBody) -> CdromCreateRequest {
        CdromCreateRequest {
            CDROM,
        }
    }
}



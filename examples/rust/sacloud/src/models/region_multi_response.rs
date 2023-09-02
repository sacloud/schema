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
pub struct RegionMultiResponse {
    #[serde(rename = "Count")]
    pub count: i32,
    #[serde(rename = "From")]
    pub from: i32,
    #[serde(rename = "Total")]
    pub total: i32,
    #[serde(rename = "is_ok")]
    pub is_ok: bool,
    #[serde(rename = "Regions")]
    pub regions: Vec<crate::models::Region>,
}

impl RegionMultiResponse {
    pub fn new(count: i32, from: i32, total: i32, is_ok: bool, regions: Vec<crate::models::Region>) -> RegionMultiResponse {
        RegionMultiResponse {
            count,
            from,
            total,
            is_ok,
            regions,
        }
    }
}



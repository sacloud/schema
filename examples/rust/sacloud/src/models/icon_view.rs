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
pub struct IconView {
    #[serde(rename = "ID")]
    pub ID: crate::models::Id,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Scope")]
    pub scope: String,
    #[serde(rename = "URL")]
    pub URL: String,
}

impl IconView {
    pub fn new(ID: crate::models::Id, name: String, scope: String, URL: String) -> IconView {
        IconView {
            ID,
            name,
            scope,
            URL,
        }
    }
}


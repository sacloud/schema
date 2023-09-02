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
pub struct Region {
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "ID")]
    pub ID: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "NameServers")]
    pub name_servers: Vec<String>,
}

impl Region {
    pub fn new(description: String, ID: i32, name: String, name_servers: Vec<String>) -> Region {
        Region {
            description,
            ID,
            name,
            name_servers,
        }
    }
}


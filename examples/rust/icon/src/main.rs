use reqwest;
use sacloud::apis::configuration::BasicAuth;
use sacloud::apis::configuration::Configuration;
use sacloud::apis::icons_api;
use sacloud::models::{icon_find_filter, icon_find_request, Scope};
use std::env;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let token = env::var("SAKURACLOUD_ACCESS_TOKEN").unwrap();
    let secret = env::var("SAKURACLOUD_ACCESS_TOKEN_SECRET").unwrap();

    let config = Configuration {
        base_path: "https://secure.sakura.ad.jp/cloud/zone/is1a/api/cloud/1.1".to_owned(),
        user_agent: Some("OpenAPI-Generator/0.0.1/rust".to_owned()),
        client: reqwest::Client::new(),
        basic_auth: Some((token, Some(secret))),
        oauth_access_token: None,
        bearer_access_token: None,
        api_key: None,
    };

    let params = icon_find_request::IconFindRequest {
        count: None,
        exclude: None,
        from: None,
        include: None,
        sort: None,
        filter: Some(icon_find_filter::IconFindFilter {
            scope: Some(Scope::User),
            name: None,
            tags_name: None,
        }),
    };

    let results = icons_api::list_icons(&config, params).await;
    match results {
        Ok(v) => println!("{:?}", v),
        Err(e) => println!("{}", e),
    };

    Ok(())
}

/*
 * SAKURA Cloud APIs
 *
 * This is a definitions for SAKURA Cloud APIs.
 *
 * The version of the OpenAPI document: 0.0.1
 * Contact: sacloud.users@gmail.com
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::{configuration, Error};
use crate::apis::ResponseContent;

/// struct for typed errors of method `close_ftp`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CloseFtpError {
    Status400(crate::models::ApiError),
    Status401(crate::models::ApiError),
    Status403(crate::models::ApiError),
    Status404(crate::models::ApiError),
    Status405(crate::models::ApiError),
    Status409(crate::models::ApiError),
    Status423(crate::models::ApiError),
    Status500(crate::models::ApiError),
    Status503(crate::models::ApiError),
    DefaultResponse(crate::models::ApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `create_cdrom`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateCdromError {
    Status400(crate::models::ApiError),
    Status401(crate::models::ApiError),
    Status403(crate::models::ApiError),
    Status405(crate::models::ApiError),
    Status500(crate::models::ApiError),
    Status503(crate::models::ApiError),
    DefaultResponse(crate::models::ApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `delete_cdromby_id`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteCdrombyIdError {
    Status400(crate::models::ApiError),
    Status401(crate::models::ApiError),
    Status403(crate::models::ApiError),
    Status404(crate::models::ApiError),
    Status405(crate::models::ApiError),
    Status409(crate::models::ApiError),
    Status423(crate::models::ApiError),
    Status500(crate::models::ApiError),
    Status503(crate::models::ApiError),
    DefaultResponse(crate::models::ApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `list_cdroms`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListCdromsError {
    Status400(crate::models::ApiError),
    Status401(crate::models::ApiError),
    Status403(crate::models::ApiError),
    Status405(crate::models::ApiError),
    Status500(crate::models::ApiError),
    Status503(crate::models::ApiError),
    DefaultResponse(crate::models::ApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `open_ftp`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OpenFtpError {
    Status400(crate::models::ApiError),
    Status401(crate::models::ApiError),
    Status403(crate::models::ApiError),
    Status404(crate::models::ApiError),
    Status405(crate::models::ApiError),
    Status409(crate::models::ApiError),
    Status423(crate::models::ApiError),
    Status500(crate::models::ApiError),
    Status503(crate::models::ApiError),
    DefaultResponse(crate::models::ApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `show_cdromby_id`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ShowCdrombyIdError {
    Status400(crate::models::ApiError),
    Status401(crate::models::ApiError),
    Status403(crate::models::ApiError),
    Status404(crate::models::ApiError),
    Status500(crate::models::ApiError),
    Status503(crate::models::ApiError),
    DefaultResponse(crate::models::ApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `update_cdromby_id`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateCdrombyIdError {
    Status400(crate::models::ApiError),
    Status401(crate::models::ApiError),
    Status403(crate::models::ApiError),
    Status404(crate::models::ApiError),
    Status405(crate::models::ApiError),
    Status409(crate::models::ApiError),
    Status423(crate::models::ApiError),
    Status500(crate::models::ApiError),
    Status503(crate::models::ApiError),
    DefaultResponse(crate::models::ApiError),
    UnknownValue(serde_json::Value),
}

/// Close FTP connection
pub async fn close_ftp(
    configuration: &configuration::Configuration,
    cdrom_id: crate::models::Id,
) -> Result<crate::models::ActionResultEnvelope, Error<CloseFtpError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/cdrom/{cdromId}/ftp",
        configuration.base_path,
        cdromId = cdrom_id
    );
    let mut local_var_req_builder = local_var_client.delete(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CloseFtpError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Create a CD-ROM
pub async fn create_cdrom(
    configuration: &configuration::Configuration,
    cdrom_create_request: crate::models::CdromCreateRequest,
) -> Result<crate::models::CdromCreateResponse, Error<CreateCdromError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/cdrom", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };
    local_var_req_builder = local_var_req_builder.json(&cdrom_create_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateCdromError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// DeleteCDROM
pub async fn delete_cdromby_id(
    configuration: &configuration::Configuration,
    cdrom_id: crate::models::Id,
) -> Result<crate::models::CdromSingleResponse, Error<DeleteCdrombyIdError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/cdrom/{cdromId}",
        configuration.base_path,
        cdromId = cdrom_id
    );
    let mut local_var_req_builder = local_var_client.delete(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DeleteCdrombyIdError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List all CD-ROM
pub async fn list_cdroms(
    configuration: &configuration::Configuration,
    params: crate::models::CdromFindRequest,
) -> Result<crate::models::CdromMultiResponse, Error<ListCdromsError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/cdrom?{params}",
        configuration.base_path,
        params = params
    );
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ListCdromsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Open FTP connection
pub async fn open_ftp(
    configuration: &configuration::Configuration,
    cdrom_id: crate::models::Id,
) -> Result<crate::models::ActionResultEnvelope, Error<OpenFtpError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/cdrom/{cdromId}/ftp",
        configuration.base_path,
        cdromId = cdrom_id
    );
    let mut local_var_req_builder = local_var_client.put(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<OpenFtpError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Info for a specific CD-ROM
pub async fn show_cdromby_id(
    configuration: &configuration::Configuration,
    cdrom_id: crate::models::Id,
) -> Result<crate::models::CdromSingleResponse, Error<ShowCdrombyIdError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/cdrom/{cdromId}",
        configuration.base_path,
        cdromId = cdrom_id
    );
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ShowCdrombyIdError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// UpdateCDROM
pub async fn update_cdromby_id(
    configuration: &configuration::Configuration,
    cdrom_id: crate::models::Id,
    cdrom_update_request: crate::models::CdromUpdateRequest,
) -> Result<crate::models::CdromSingleResponse, Error<UpdateCdrombyIdError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/cdrom/{cdromId}",
        configuration.base_path,
        cdromId = cdrom_id
    );
    let mut local_var_req_builder = local_var_client.put(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };
    local_var_req_builder = local_var_req_builder.json(&cdrom_update_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateCdrombyIdError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

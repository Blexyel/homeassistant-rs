//! Implements the Homeassistant API for use in rust
//!
//! A simple lib, that queries different endpoints and returns the data in a usable format.
//!
//! The first 2 arguments of each function are always: `HA_URL`, `API_Token`.
//!
//! These arguments do not have to be filled with actual data, they can be `None`, but in this case you will need to use environment variables.
//!
//! Under the hood we use dotenvy.
//!
//! Example env:
//! ```text
//! HA_URL="http://localhost:8123"
//! HA_TOKEN="api_token_from_hass"
//! ```
//!
//! - Easily get HA's config:
//! ```
//! # use tokio::runtime::Runtime;
//! # let rt = Runtime::new().unwrap();
//! # rt.block_on(async {
//! use homeassistant_rs::{self, hass};
//! let config = hass().config(None, None).await.unwrap();
//!
//! println!("{}", config.version);
//! # });
//! ```
//!
//! You can check all available endpoints here: [`HomeAssistant`]
//!
//! - More Examples:
//!
//!
//! ```
//! # use tokio::runtime::Runtime;
//! # let rt = Runtime::new().unwrap();
//! # rt.block_on(async {
//! use homeassistant_rs::hass;
//! 
//! hass().config(None, None).await.unwrap();
//! hass().events(None, None).await.unwrap();
//! hass().services(None, None).await.unwrap();
//! hass()
//!     .history(
//!         None,
//!         None,
//!         Some("light.bedroom_local_bedroom_local"),
//!         /// minimal_response
//!         true,
//!         /// no_attributes
//!         true,
//!         /// significant_changes_only
//!         true,
//!     )
//!     .await.unwrap();
//! hass().logbook(None, None, Some("light.bedroom_local_bedroom_local")).await.unwrap();
//! hass().states(None, None, Some("light.bedroom_local_bedroom_local")).await.unwrap();
//! hass().states(None, None, None).await.unwrap();
//! hass().error_log(None, None).await.unwrap();
//!  # });
//! ```

#[cfg(test)]
mod tests;
pub use ::bytes;
pub use ::lazy_static;
pub use ::reqwest;
pub use ::serde;
pub use ::serde_json;
use serde_json::json;

pub mod structs;

// ### BEGIN INTERNAL USE ONLY ###

lazy_static::lazy_static! {
    pub static ref CLIENT: reqwest::Client = reqwest::Client::new();

    static ref GLOBAL_VARS: GlobalVars = GlobalVars::new();
}

struct GlobalVars {
    url: Option<String>,
    token: Option<String>,
}

impl GlobalVars {
    fn new() -> Self {
        Self {
            url: dotenvy::var("HA_URL").ok(),
            token: dotenvy::var("HA_TOKEN").ok(),
        }
    }
}

fn globalvars() -> &'static GlobalVars {
    GlobalVars::new();
    &GLOBAL_VARS
}

struct Validate;

impl Validate {
    fn arg(&self, str: Option<String>) -> anyhow::Result<String, anyhow::Error> {
        if let Some(str) = str {
            Ok(str)
        } else {
            Err(anyhow::Error::msg("Seems empty"))
        }
    }
}

fn validate() -> Validate {
    Validate
}

async fn request(url: String, token: String, path: &str) -> anyhow::Result<reqwest::Response> {
    Ok(CLIENT
        .get(url.to_owned() + path)
        .bearer_auth(token)
        .send()
        .await?)
}

async fn post<T: serde::Serialize>(
    url: String,
    token: String,
    path: &str,
    json: T,
) -> anyhow::Result<reqwest::Response> {
    if !serde_json::to_string(&json)?.is_empty() {
        Ok(CLIENT
            .post(url.to_owned() + path)
            .bearer_auth(token)
            .json(&json)
            .send()
            .await?)
    } else {
        Ok(CLIENT
            .post(url.to_owned() + path)
            .bearer_auth(token)
            .send()
            .await?)
    }
}

// ### END INTERNAL USE ONLY ###

pub struct HomeAssistant;

impl HomeAssistant {
    pub fn request(&self) -> &'static HomeAssistantPost {
        &HomeAssistantPost
    }

    /// queries `/api/config` and returns [`ConfigResponse`](structs::ConfigResponse) struct
    pub async fn config(
        &self,
        ha_url: Option<String>,
        ha_token: Option<String>,
    ) -> anyhow::Result<structs::ConfigResponse> {
        let vars = globalvars();
        let url = validate().arg(ha_url).or_else(|_| {
            vars.url
                .clone()
                .ok_or(anyhow::Error::msg("HA_URL is required"))
        })?;
        let token = validate().arg(ha_token).or_else(|_| {
            vars.token
                .clone()
                .ok_or(anyhow::Error::msg("HA_TOKEN is required"))
        })?;

        let client = request(url, token, "/api/config").await?;
        if !client.status().is_success() {
            Err(anyhow::Error::msg(client.status()))
        } else {
            Ok(client.json::<structs::ConfigResponse>().await?)
        }
    }

    /// queries `/api/events` and returns a Vec containing [`EventResponse`](structs::EventResponse) struct    
    pub async fn events(
        &self,
        ha_url: Option<String>,
        ha_token: Option<String>,
    ) -> anyhow::Result<Vec<structs::EventResponse>> {
        let vars = globalvars();
        let url = validate().arg(ha_url).or_else(|_| {
            vars.url
                .clone()
                .ok_or(anyhow::Error::msg("HA_URL is required"))
        })?;
        let token = validate().arg(ha_token).or_else(|_| {
            vars.token
                .clone()
                .ok_or(anyhow::Error::msg("HA_TOKEN is required"))
        })?;

        let client = request(url, token, "/api/events").await?;

        if !client.status().is_success() {
            Err(anyhow::Error::msg(client.status()))
        } else {
            Ok(client.json::<Vec<structs::EventResponse>>().await?)
        }
    }

    /// queries `/api/services` and returns a Vec containing [`Value`](serde_json::Value) (subject to possibly change in the future)
    pub async fn services(
        &self,
        ha_url: Option<String>,
        ha_token: Option<String>,
    ) -> anyhow::Result<Vec<serde_json::Value>> {
        let vars = globalvars();
        let url = validate().arg(ha_url).or_else(|_| {
            vars.url
                .clone()
                .ok_or(anyhow::Error::msg("HA_URL is required"))
        })?;
        let token = validate().arg(ha_token).or_else(|_| {
            vars.token
                .clone()
                .ok_or(anyhow::Error::msg("HA_TOKEN is required"))
        })?;

        let client = request(url, token, "/api/services").await?.bytes().await?;

        let services: Vec<serde_json::Value> = serde_json::from_slice(&client)?;

        Ok(services)
    }

    /// queries `/api/history/period/<optionalargs>` and returns a Vec containing [`HistoryResponse`](structs::HistoryResponse) struct
    pub async fn history(
        &self,
        ha_url: Option<String>,
        ha_token: Option<String>,
        ha_entity_id: Option<&str>,
        minimal_response: bool,
        no_attributes: bool,
        significant_changes_only: bool,
    ) -> anyhow::Result<Vec<structs::HistoryResponse>> {
        let vars = globalvars();
        let url = validate().arg(ha_url).or_else(|_| {
            vars.url
                .clone()
                .ok_or(anyhow::Error::msg("HA_URL is required"))
        })?;
        let token = validate().arg(ha_token).or_else(|_| {
            vars.token
                .clone()
                .ok_or(anyhow::Error::msg("HA_TOKEN is required"))
        })?;

        let path = format!(
            "?filter_entity_id={0}{1}{2}{3}",
            ha_entity_id.unwrap_or(""),
            if minimal_response {
                "&minimal_response"
            } else {
                ""
            },
            if no_attributes { "&no_attributes" } else { "" },
            if significant_changes_only {
                "&significant_changes_only"
            } else {
                ""
            }
        );

        let client = request(url, token, &format!("/api/history/period{path}")).await?;

        if !client.status().is_success() {
            Err(anyhow::Error::msg(client.status()))
        } else {
            Ok(client
                .json::<Vec<Vec<structs::HistoryResponse>>>()
                .await?
                .into_iter()
                .flatten()
                .collect())
        }
    }

    /// queries `/api/logbook` and returns a Vec containing [`LogBook`](structs::LogBook) struct
    pub async fn logbook(
        &self,
        ha_url: Option<String>,
        ha_token: Option<String>,
        ha_entity_id: Option<&str>,
    ) -> anyhow::Result<Vec<structs::LogBook>> {
        let vars = globalvars();
        let url = validate().arg(ha_url).or_else(|_| {
            vars.url
                .clone()
                .ok_or(anyhow::Error::msg("HA_URL is required"))
        })?;
        let token = validate().arg(ha_token).or_else(|_| {
            vars.token
                .clone()
                .ok_or(anyhow::Error::msg("HA_TOKEN is required"))
        })?;

        let client = request(
            url,
            token,
            &format!(
                "/api/logbook{0}",
                ("?".to_owned() + ha_entity_id.unwrap_or(""))
            ),
        )
        .await?;
        if !client.status().is_success() {
            Err(anyhow::Error::msg(client.status()))
        } else {
            Ok(client.json::<Vec<structs::LogBook>>().await?)
        }
    }

    /// queries `/api/states/<optional_entity_id>` and returns a Vec containing [`StatesResponse`](structs::StatesResponse) struct
    pub async fn states(
        &self,
        ha_url: Option<String>,
        ha_token: Option<String>,
        ha_entity_id: Option<&str>,
    ) -> anyhow::Result<Vec<structs::StatesResponse>> {
        let vars = globalvars();
        let url = validate().arg(ha_url).or_else(|_| {
            vars.url
                .clone()
                .ok_or(anyhow::Error::msg("HA_URL is required"))
        })?;
        let token = validate().arg(ha_token).or_else(|_| {
            vars.token
                .clone()
                .ok_or(anyhow::Error::msg("HA_TOKEN is required"))
        })?;

        let entity_id = ha_entity_id.unwrap_or_default();

        let client = if entity_id.is_empty() {
            request(url, token, "/api/states")
                .await?
                .json::<Vec<structs::StatesResponse>>()
                .await?
        } else {
            vec![
                request(url, token, &format!("/api/states/{entity_id}"))
                    .await?
                    .json::<structs::StatesResponse>()
                    .await?,
            ]
        };

        Ok(client)
    }

    /// queries `/api/error_log` and returns a [`String`]
    pub async fn error_log(
        &self,
        ha_url: Option<String>,
        ha_token: Option<String>,
    ) -> anyhow::Result<String> {
        let vars = globalvars();
        let url = validate().arg(ha_url).or_else(|_| {
            vars.url
                .clone()
                .ok_or(anyhow::Error::msg("HA_URL is required"))
        })?;
        let token = validate().arg(ha_token).or_else(|_| {
            vars.token
                .clone()
                .ok_or(anyhow::Error::msg("HA_TOKEN is required"))
        })?;

        let client = request(url, token, "/api/states").await?.text().await?;

        Ok(client)
    }

    /// queries `/api/camera_proxy/<camera_entity_id>?time=<timestamp>` and returns [`Bytes`](bytes::Bytes)
    ///
    /// input parameter `time` as `unix_time` in seconds ([`u64`])
    ///
    /// <sub>WARNING: Further testing is required for this function, as i (Blexyel) am not able to test it myself</sub>
    pub async fn camera_proxy(
        &self,
        ha_url: Option<String>,
        ha_token: Option<String>,
        ha_entity_id: &str,
        time: u64,
    ) -> anyhow::Result<bytes::Bytes> {
        let vars = globalvars();
        let url = validate().arg(ha_url).or_else(|_| {
            vars.url
                .clone()
                .ok_or(anyhow::Error::msg("HA_URL is required"))
        })?;
        let token = validate().arg(ha_token).or_else(|_| {
            vars.token
                .clone()
                .ok_or(anyhow::Error::msg("HA_TOKEN is required"))
        })?;

        let client = request(
            url,
            token,
            &format!("/api/camera_proxy/{ha_entity_id}?time={time}"),
        )
        .await?
        .bytes()
        .await?;

        Ok(client)
    }

    /// queries `/api/calendars/<calendar entity_id>?start=<timestamp>&end=<timestamp>` and returns a Vec containing `[CalendarResponse`](structs::CalendarResponse)
    #[allow(unreachable_code, unused_variables)]
    pub async fn calendars(
        &self,
        ha_url: Option<String>,
        ha_token: Option<String>,
    ) -> anyhow::Result<Vec<structs::CalendarResponse>> {
        unimplemented!(
            "I (Blexyel) am unable to implement this function, as (apparently) my HASS instance does not have calendars. Feel free to make a PR to implement this feature"
        );
        {
            let vars = globalvars();
            let url = validate().arg(ha_url).or_else(|_| {
                vars.url
                    .clone()
                    .ok_or(anyhow::Error::msg("HA_URL is required"))
            })?;
            let token = validate().arg(ha_token).or_else(|_| {
                vars.token
                    .clone()
                    .ok_or(anyhow::Error::msg("HA_TOKEN is required"))
            })?;

            let client = request(url, token, "/api/calendars").await?.bytes().await?;

            Ok(vec![structs::CalendarResponse {
                entity_id: todo!(),
                name: todo!(),
            }])
        }
    }
}

pub struct HomeAssistantPost;

impl HomeAssistantPost {
    /// posts to `/api/states/<entity_id>` to update/create a state and returns [`StatesResponse`](structs::StatesResponse)
    pub async fn state(
        &self,
        ha_url: Option<String>,
        ha_token: Option<String>,
        ha_entity_id: &str,
        request: structs::StatesRequest,
    ) -> anyhow::Result<structs::StatesResponse> {
        let vars = globalvars();
        let url = validate().arg(ha_url).or_else(|_| {
            vars.url
                .clone()
                .ok_or(anyhow::Error::msg("HA_URL is required"))
        })?;
        let token = validate().arg(ha_token).or_else(|_| {
            vars.token
                .clone()
                .ok_or(anyhow::Error::msg("HA_TOKEN is required"))
        })?;

        let client = post(url, token, &format!("/api/states/{ha_entity_id}"), request).await?;
        if !client.status().is_success() {
            Err(anyhow::Error::msg(client.status()))
        } else {
            Ok(client.json::<structs::StatesResponse>().await?)
        }
    }
    // I have been programming for ~7 Hours straight, I'm tired

    /// posts to `/api/events/<event_type>` to update/create a state and returns [`StatesResponse`](structs::StatesResponse)
    ///
    /// request param does not need to have data, it can be empty, e.g.:
    /// ```ignore
    /// json!({})
    /// ```
    pub async fn events(
        &self,
        ha_url: Option<String>,
        ha_token: Option<String>,
        ha_event_type: &str,
        request: serde_json::Value,
    ) -> anyhow::Result<structs::SimpleResponse> {
        let vars = globalvars();
        let url = validate().arg(ha_url).or_else(|_| {
            vars.url
                .clone()
                .ok_or(anyhow::Error::msg("HA_URL is required"))
        })?;
        let token = validate().arg(ha_token).or_else(|_| {
            vars.token
                .clone()
                .ok_or(anyhow::Error::msg("HA_TOKEN is required"))
        })?;

        let client = post(url, token, &format!("/api/events/{ha_event_type}"), request).await?;

        if !client.status().is_success() {
            Err(anyhow::Error::msg(client.status()))
        } else {
            Ok(client.json::<structs::SimpleResponse>().await?)
        }
    }

    /// posts to `/api/services/<domain>/<service>` to call a service within a specific domain and returns [`Value`](serde_json::Value)
    ///
    /// request param does not need to have data, it can be empty, e.g.:
    /// ```ignore
    /// json!({})
    /// ```
    pub async fn service(
        &self,
        ha_url: Option<String>,
        ha_token: Option<String>,
        ha_domain: &str,
        ha_service: &str,
        request: serde_json::Value,
        return_response: bool,
    ) -> anyhow::Result<serde_json::Value> {
        let vars = globalvars();
        let url = validate().arg(ha_url).or_else(|_| {
            vars.url
                .clone()
                .ok_or(anyhow::Error::msg("HA_URL is required"))
        })?;
        let token = validate().arg(ha_token).or_else(|_| {
            vars.token
                .clone()
                .ok_or(anyhow::Error::msg("HA_TOKEN is required"))
        })?;

        let client = post(
            url,
            token,
            &format!(
                "/api/services/{ha_domain}/{ha_service}{0}",
                if return_response {
                    "?return_response"
                } else {
                    ""
                }
            ),
            request,
        )
        .await?;

        if !client.status().is_success() {
            Err(anyhow::Error::msg(client.status()))
        } else {
            Ok(client.json::<serde_json::Value>().await?)
        }
    }

    /// posts to `/api/template` and renders a HASS template and returns [`String`]
    pub async fn template(
        &self,
        ha_url: Option<String>,
        ha_token: Option<String>,
        request: structs::TemplateRequest,
    ) -> anyhow::Result<String> {
        let vars = globalvars();
        let url = validate().arg(ha_url).or_else(|_| {
            vars.url
                .clone()
                .ok_or(anyhow::Error::msg("HA_URL is required"))
        })?;
        let token = validate().arg(ha_token).or_else(|_| {
            vars.token
                .clone()
                .ok_or(anyhow::Error::msg("HA_TOKEN is required"))
        })?;

        let client = post(url, token, "/api/template", request)
            .await?
            .text()
            .await?;

        Ok(client)
    }

    /// posts to `/api/config/core/check_config` and checks the config and returns [`ConfigCheckResponse`](structs::ConfigCheckResponse)
    pub async fn config_check(
        &self,
        ha_url: Option<String>,
        ha_token: Option<String>,
    ) -> anyhow::Result<structs::ConfigCheckResponse> {
        let vars = globalvars();
        let url = validate().arg(ha_url).or_else(|_| {
            vars.url
                .clone()
                .ok_or(anyhow::Error::msg("HA_URL is required"))
        })?;
        let token = validate().arg(ha_token).or_else(|_| {
            vars.token
                .clone()
                .ok_or(anyhow::Error::msg("HA_TOKEN is required"))
        })?;

        let client = post(url, token, "/api/config/core/check_config", json!({})).await?;

        if !client.status().is_success() {
            Err(anyhow::Error::msg(client.status()))
        } else {
            Ok(client.json::<structs::ConfigCheckResponse>().await?)
        }
    }

    /// posts to `/api/intent/handle` and handles an Intent and returns a [`String`]
    ///
    /// I (Blexyel) am unable to test this function
    pub async fn intent(
        &self,
        ha_url: Option<String>,
        ha_token: Option<String>,
        request: serde_json::Value,
    ) -> anyhow::Result<String> {
        let vars = globalvars();
        let url = validate().arg(ha_url).or_else(|_| {
            vars.url
                .clone()
                .ok_or(anyhow::Error::msg("HA_URL is required"))
        })?;
        let token = validate().arg(ha_token).or_else(|_| {
            vars.token
                .clone()
                .ok_or(anyhow::Error::msg("HA_TOKEN is required"))
        })?;

        let client = post(url, token, "/api/intent/handle", request)
            .await?
            .text()
            .await?;

        Ok(client)
    }
}

pub fn hass() -> HomeAssistant {
    HomeAssistant
}
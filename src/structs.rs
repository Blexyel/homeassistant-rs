use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone, Default)]
pub struct ConfigResponse {
    pub components: Vec<String>,
    pub config_dir: String,
    pub elevation: f64,
    pub latitude: f64,
    pub location_name: String,
    pub longitude: f64,
    pub time_zone: String,
    pub unit_system: UnitSystem,
    pub version: String,
    pub whitelist_external_dirs: Vec<String>,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct UnitSystem {
    pub length: String,
    pub mass: String,
    pub temperature: String,
    pub volume: String,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct EventResponse {
    pub event: String,
    pub listener_count: u16,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct HistoryResponse {
    pub entity_id: Option<String>,
    pub state: String,
    pub attributes: Option<Attributes>,
    pub last_changed: String,
    pub last_updated: Option<String>,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct Attributes {
    pub friendly_name: Option<String>,
    pub editable: Option<bool>,
    pub id: Option<String>,
    pub source: Option<String>,
    pub user_id: Option<String>,
    pub icon: Option<String>,
    #[serde(flatten)]
    pub other_fields: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct LogBook {
    pub name: String,
    pub message: Option<String>,
    pub source: Option<String>,
    pub entity_id: String,
    #[serde(alias = "context_id", alias = "context_user_id")]
    pub context_id: Option<String>,
    pub domain: Option<String>,
    pub when: String,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct StatesResponse {
    pub entity_id: Option<String>,
    pub state: String,
    pub attributes: Option<Attributes>,
    pub last_changed: Option<String>,
    pub last_reported: Option<String>,
    pub last_updated: Option<String>,
    pub context: Option<Context>,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct Context {
    pub id: String,
    pub parent_id: Option<String>,
    pub user_id: Option<String>,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct CalendarResponse {
    pub entity_id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct StatesRequest {
    pub state: String,
    #[serde(flatten)]
    pub attributes: Option<serde_json::Value>,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct SimpleResponse {
    pub message: String,
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct TemplateRequest {
    pub template: String,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct ConfigCheckResponse {
    pub errors: Option<String>,
    pub result: String,
    pub warnings: Option<String>,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct ServicesResponse {
    pub domain: String,
    pub services: serde_json::Value,
}
//! Local stub for cloud search (disabled for local-first Memora architecture).

use chrono::{DateTime, Utc};
use oasgen::OaSchema;
use serde::{Deserialize, Serialize};

/// Cloud search client (stubbed out for local-first operation).
pub struct CloudSearchClient;

impl CloudSearchClient {
    pub fn new() -> Self {
        Self
    }

    pub async fn is_enabled(&self) -> bool {
        false
    }

    pub async fn set_enabled(&self, _enabled: bool) {}

    pub async fn search(&self, _params: CloudSearchParams) -> Result<CloudSearchResult, String> {
        Ok(CloudSearchResult {
            items: vec![],
            has_more: false,
            cloud_status: CloudStatus::Disabled,
            latency_ms: 0,
        })
    }

    pub async fn get_metadata(
        &self,
        _query: &str,
        _time_range: Option<TimeRange>,
    ) -> CloudSearchMetadata {
        CloudSearchMetadata {
            cloud_search_available: false,
            cloud_has_relevant_data: false,
            cloud_query_hint: None,
        }
    }
}

impl Default for CloudSearchClient {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, OaSchema)]
pub enum CloudStatus {
    Disabled,
    NotInitialized,
    Available,
    Error(String),
}

#[derive(Debug, Clone)]
pub struct TimeRange {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct CloudSearchParams {
    pub query: String,
    pub limit: usize,
    pub time_range: Option<TimeRange>,
}

#[derive(Debug, Clone, Serialize, Deserialize, OaSchema)]
pub struct CloudSearchResultItem {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub source: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, OaSchema)]
pub struct CloudSearchResult {
    pub items: Vec<CloudSearchResultItem>,
    pub has_more: bool,
    pub cloud_status: CloudStatus,
    pub latency_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, OaSchema)]
pub struct CloudSearchMetadata {
    pub cloud_search_available: bool,
    pub cloud_has_relevant_data: bool,
    pub cloud_query_hint: Option<String>,
}

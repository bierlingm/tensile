#[cfg(feature = "cloud")]
use crate::{error::TensileError, models::Database, TensileResult};

#[cfg(feature = "cloud")]
pub struct CloudSync {
    config: super::CloudConfig,
}

#[cfg(feature = "cloud")]
impl CloudSync {
    pub fn new(config: super::CloudConfig) -> Self {
        CloudSync { config }
    }

    pub async fn push(&self, db: &Database) -> TensileResult<()> {
        let payload =
            serde_json::to_string(db).map_err(|e| TensileError::Serialization(e.to_string()))?;

        let client = reqwest::Client::new();
        let response = client
            .post(format!("{}/sync/push", self.config.api_url))
            .bearer_auth(&self.config.api_token)
            .header("X-User-ID", &self.config.user_id)
            .body(payload)
            .send()
            .await
            .map_err(|e| TensileError::Unknown(e.to_string()))?;

        if !response.status().is_success() {
            return Err(TensileError::Unknown(format!(
                "Cloud sync failed: {}",
                response.status()
            )));
        }

        Ok(())
    }

    pub async fn pull(&self) -> TensileResult<Database> {
        let client = reqwest::Client::new();
        let response = client
            .get(format!("{}/sync/pull", self.config.api_url))
            .bearer_auth(&self.config.api_token)
            .header("X-User-ID", &self.config.user_id)
            .send()
            .await
            .map_err(|e| TensileError::Unknown(e.to_string()))?;

        if !response.status().is_success() {
            return Err(TensileError::Unknown(format!(
                "Cloud sync failed: {}",
                response.status()
            )));
        }

        let body = response
            .text()
            .await
            .map_err(|e| TensileError::Unknown(e.to_string()))?;

        serde_json::from_str(&body).map_err(|e| TensileError::Serialization(e.to_string()))
    }
}

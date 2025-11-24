#[cfg(feature = "cloud")]
pub mod sync;

#[cfg(feature = "cloud")]
pub use sync::CloudSync;

#[cfg(feature = "cloud")]
pub struct CloudConfig {
    pub api_url: String,
    pub api_token: String,
    pub user_id: String,
}

#[cfg(feature = "cloud")]
impl CloudConfig {
    pub fn from_env() -> crate::TensileResult<Self> {
        let api_url = std::env::var("TENSILE_CLOUD_URL")
            .unwrap_or_else(|_| "https://api.tensile.dev".to_string());
        let api_token = std::env::var("TENSILE_CLOUD_TOKEN").map_err(|_| {
            crate::error::TensileError::Validation("TENSILE_CLOUD_TOKEN not set".to_string())
        })?;
        let user_id = std::env::var("TENSILE_USER_ID").map_err(|_| {
            crate::error::TensileError::Validation("TENSILE_USER_ID not set".to_string())
        })?;

        Ok(CloudConfig {
            api_url,
            api_token,
            user_id,
        })
    }
}

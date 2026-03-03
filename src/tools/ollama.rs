//! Ollama client for local LLM inference

use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Clone)]
pub struct OllamaClient {
    client: Client,
    base_url: String,
}

#[derive(Debug, Serialize)]
pub struct OllamaRequest {
    pub model: String,
    pub prompt: String,
    pub stream: bool,
}

#[derive(Debug, Deserialize)]
pub struct OllamaResponse {
    pub response: String,
    pub done: bool,
}

impl OllamaClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(60))
                .build()
                .expect("failed to build reqwest client"),
            base_url: base_url.to_string(),
        }
    }

    pub async fn generate(&self, model: &str, prompt: &str) -> Result<String> {
        let request = OllamaRequest {
            model: model.to_string(),
            prompt: prompt.to_string(),
            stream: false,
        };

        let response = self
            .client
            .post(format!("{}/api/generate", self.base_url))
            .json(&request)
            .send()
            .await?
            .error_for_status()?
            .json::<OllamaResponse>()
            .await?;

        Ok(response.response)
    }

    pub async fn list_models(&self) -> Result<Vec<String>> {
        #[derive(Deserialize)]
        struct ModelList {
            models: Vec<ModelInfo>,
        }

        #[derive(Deserialize)]
        struct ModelInfo {
            name: String,
        }

        let response = self
            .client
            .get(format!("{}/api/tags", self.base_url))
            .send()
            .await?
            .error_for_status()?
            .json::<ModelList>()
            .await?;

        Ok(response.models.iter().map(|m| m.name.clone()).collect())
    }
}

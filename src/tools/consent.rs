//! Consent management for human-in-the-loop privacy decisions

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentRequest {
    pub id: String,
    pub description: String,
    pub data_type: String,
    pub destination: String,
    pub created_at: u64,
    pub expires_at: u64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ConsentStatus {
    Pending,
    Granted,
    Denied,
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentRecord {
    pub request: ConsentRequest,
    pub status: ConsentStatus,
    pub responded_at: Option<u64>,
}

pub struct ConsentManager {
    pending_requests: Vec<ConsentRequest>,
    records: Vec<ConsentRecord>,
}

impl ConsentManager {
    pub fn new() -> Self {
        Self {
            pending_requests: vec![],
            records: vec![],
        }
    }

    pub fn request_consent(
        &mut self,
        description: &str,
        data_type: &str,
        destination: &str,
    ) -> ConsentRequest {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        let request = ConsentRequest {
            id: uuid::Uuid::new_v4().to_string(),
            description: description.to_string(),
            data_type: data_type.to_string(),
            destination: destination.to_string(),
            created_at: now,
            expires_at: now + 3600, // 1 hour expiry
        };

        self.pending_requests.push(request.clone());
        request
    }

    pub fn grant(&mut self, request_id: &str) -> Result<()> {
        let index = self
            .pending_requests
            .iter()
            .position(|request| request.id == request_id)
            .ok_or_else(|| anyhow::anyhow!("Consent request not found: {}", request_id))?;

        let request = self.pending_requests.remove(index);
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        self.records.push(ConsentRecord {
            request,
            status: ConsentStatus::Granted,
            responded_at: Some(now),
        });

        Ok(())
    }

    pub fn deny(&mut self, request_id: &str) -> Result<()> {
        let index = self
            .pending_requests
            .iter()
            .position(|request| request.id == request_id)
            .ok_or_else(|| anyhow::anyhow!("Consent request not found: {}", request_id))?;

        let request = self.pending_requests.remove(index);
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        self.records.push(ConsentRecord {
            request,
            status: ConsentStatus::Denied,
            responded_at: Some(now),
        });

        Ok(())
    }

    pub fn get_pending(&self) -> &[ConsentRequest] {
        &self.pending_requests
    }

    pub fn records(&self) -> &[ConsentRecord] {
        &self.records
    }
}

impl Default for ConsentManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grant_and_deny_move_requests_to_records() {
        let mut manager = ConsentManager::new();
        let grant_req = manager.request_consent("Send to cloud", "email", "openai");
        let deny_req = manager.request_consent("Send to cloud", "medical", "openai");

        manager.grant(&grant_req.id).expect("grant should succeed");
        manager.deny(&deny_req.id).expect("deny should succeed");

        assert_eq!(manager.get_pending().len(), 0);
        assert_eq!(manager.records().len(), 2);
        assert!(matches!(
            manager.records()[0].status,
            ConsentStatus::Granted
        ));
        assert!(matches!(manager.records()[1].status, ConsentStatus::Denied));
    }

    #[test]
    fn grant_unknown_request_fails() {
        let mut manager = ConsentManager::new();
        let err = manager.grant("missing-id").expect_err("should fail");
        assert!(err.to_string().contains("Consent request not found"));
    }
}

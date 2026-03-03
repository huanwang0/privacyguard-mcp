//! Audit logging for compliance tracking

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::fs;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    pub timestamp: u64,
    pub event_type: EventType,
    pub agent_id: Option<String>,
    pub action: String,
    pub privacy_level: String,
    pub route_decision: String,
    pub consent_required: bool,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    PrivacyCheck,
    ConsentRequest,
    ConsentGranted,
    ConsentDenied,
    RouteToLocal,
    RouteToCloud,
    PolicyViolation,
}

#[derive(Clone)]
pub struct AuditLogger {
    log_path: String,
}

impl AuditLogger {
    pub fn new(log_path: &str) -> Self {
        Self {
            log_path: log_path.to_string(),
        }
    }

    pub async fn log(&self, event: AuditEvent) -> Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_path)
            .await?;

        let json = serde_json::to_string(&event)?;
        file.write_all(json.as_bytes()).await?;
        file.write_all(b"\n").await?;

        Ok(())
    }

    pub async fn log_privacy_check(
        &self,
        agent_id: Option<&str>,
        action: &str,
        privacy_level: &str,
        route_decision: &str,
    ) -> Result<()> {
        let event = AuditEvent {
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0),
            event_type: EventType::PrivacyCheck,
            agent_id: agent_id.map(String::from),
            action: action.to_string(),
            privacy_level: privacy_level.to_string(),
            route_decision: route_decision.to_string(),
            consent_required: false,
            metadata: serde_json::json!({}),
        };

        self.log(event).await
    }
}

pub async fn read_recent_events(log_path: &str, limit: usize) -> Result<Vec<AuditEvent>> {
    let content = match fs::read_to_string(log_path).await {
        Ok(content) => content,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(vec![]),
        Err(e) => return Err(e.into()),
    };

    let mut events = Vec::new();
    for line in content.lines().rev() {
        if line.trim().is_empty() {
            continue;
        }

        if let Ok(event) = serde_json::from_str::<AuditEvent>(line) {
            events.push(event);
        }

        if events.len() >= limit {
            break;
        }
    }

    events.reverse();
    Ok(events)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::fs;

    #[tokio::test]
    async fn reads_recent_events_in_order() {
        let path =
            std::env::temp_dir().join(format!("privacyguard-audit-{}.log", uuid::Uuid::new_v4()));
        let logger = AuditLogger::new(path.to_str().expect("temp path should be utf8"));

        logger
            .log(AuditEvent {
                timestamp: 1,
                event_type: EventType::PrivacyCheck,
                agent_id: None,
                action: "a".to_string(),
                privacy_level: "public".to_string(),
                route_decision: "cloud".to_string(),
                consent_required: false,
                metadata: serde_json::json!({}),
            })
            .await
            .expect("log write should succeed");
        logger
            .log(AuditEvent {
                timestamp: 2,
                event_type: EventType::RouteToLocal,
                agent_id: None,
                action: "b".to_string(),
                privacy_level: "sensitive".to_string(),
                route_decision: "local".to_string(),
                consent_required: true,
                metadata: serde_json::json!({}),
            })
            .await
            .expect("log write should succeed");

        let events = read_recent_events(path.to_str().expect("temp path should be utf8"), 1)
            .await
            .expect("read should succeed");
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].timestamp, 2);

        let _ = fs::remove_file(path).await;
    }
}

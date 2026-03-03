//! Privacy routing logic
//!
//! Determines whether data should be sent to cloud LLM or routed to local Ollama

use serde::{Deserialize, Serialize};

/// Privacy classification for content
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PrivacyLevel {
    /// Public information - safe for any LLM
    Public,
    /// Internal use - okay for trusted cloud providers
    Internal,
    /// Sensitive - requires explicit consent for cloud
    Sensitive,
    /// Highly sensitive - local only, no cloud
    Confidential,
}

/// Result of privacy check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyCheckResult {
    pub level: PrivacyLevel,
    pub route_to_local: bool,
    pub requires_consent: bool,
    pub reason: String,
}

/// Check privacy level of content
pub fn check_privacy(content: &str) -> PrivacyCheckResult {
    // Simple heuristic-based classification
    // TODO: Implement ML-based classifier

    let sensitive_patterns = [
        "password",
        "secret",
        "api_key",
        "token",
        "credential",
        "ssn",
        "social security",
        "credit card",
        "bank account",
        "medical",
        "diagnosis",
        "prescription",
        "hipaa",
        "salary",
        "compensation",
        "ssn",
        "passport",
    ];

    let content_lower = content.to_lowercase();

    for pattern in &sensitive_patterns {
        if content_lower.contains(pattern) {
            return PrivacyCheckResult {
                level: PrivacyLevel::Sensitive,
                route_to_local: true,
                requires_consent: true,
                reason: format!("Contains sensitive keyword: {}", pattern),
            };
        }
    }

    // Default to public for now
    PrivacyCheckResult {
        level: PrivacyLevel::Public,
        route_to_local: false,
        requires_consent: false,
        reason: "No sensitive content detected".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_public_content() {
        let result = check_privacy("What's the weather today?");
        assert_eq!(result.level, PrivacyLevel::Public);
        assert!(!result.route_to_local);
    }

    #[test]
    fn test_sensitive_content() {
        let result = check_privacy("My password is secret123");
        assert!(result.level == PrivacyLevel::Sensitive);
        assert!(result.route_to_local);
    }
}

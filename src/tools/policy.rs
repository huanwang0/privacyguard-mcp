//! Policy engine for privacy routing decisions

use crate::tools::privacy::{PrivacyCheckResult, PrivacyLevel};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    pub name: String,
    pub default_level: PrivacyLevel,
    pub rules: Vec<Rule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub pattern: String,
    pub action: RuleAction,
    pub priority: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleAction {
    RouteLocal,
    RouteCloud,
    RequireConsent,
    Block,
}

pub struct PolicyEngine {
    policies: Vec<Policy>,
}

impl PolicyEngine {
    pub fn new() -> Self {
        // TODO: load policies from config file when configuration is introduced
        Self {
            policies: vec![Policy::default()],
        }
    }

    pub fn evaluate(&self, content: &str, check_result: &PrivacyCheckResult) -> Decision {
        // Apply policy rules
        for policy in &self.policies {
            for rule in &policy.rules {
                if content.contains(&rule.pattern) {
                    return match rule.action {
                        RuleAction::RouteLocal => Decision::RouteLocal,
                        RuleAction::RouteCloud => Decision::RouteCloud,
                        RuleAction::RequireConsent => Decision::RequireConsent,
                        RuleAction::Block => Decision::Block,
                    };
                }
            }
        }

        // Default decision based on privacy check
        if check_result.route_to_local {
            Decision::RouteLocal
        } else {
            Decision::RouteCloud
        }
    }
}

impl Default for PolicyEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub enum Decision {
    RouteLocal,
    RouteCloud,
    RequireConsent,
    Block,
}

impl Default for Policy {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            default_level: PrivacyLevel::Public,
            rules: vec![],
        }
    }
}

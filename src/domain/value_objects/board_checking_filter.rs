use super::quest_statuses::QuestStatuses;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BoardCheckingFilter {
    name: Option<String>,
    status: Option<QuestStatuses>,
}

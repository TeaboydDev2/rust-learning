use serde::{Deserialize, Serialize};

use crate::domain::entities::adventurers::RegisterAdventurerEntity;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterAdventurerModel {
    username: String,
    password: String,
}

impl RegisterAdventurerModel {
    pub fn to_entity(&self) -> RegisterAdventurerEntity {
        RegisterAdventurerEntity {
            username: self.username.clone(),
            password: self.password.clone(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}

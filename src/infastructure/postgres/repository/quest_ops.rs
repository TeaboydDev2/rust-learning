use anyhow::Result;
use axum::async_trait;
use std::sync::Arc;

use crate::{
    domain::{
        entities::quests::{AddQuestEntity, EditQuestEntity},
        repository::quest_ops::QuestOpsRepository,
    },
    infastructure::postgres::postgres_connection::PgPoolSquad,
};

pub struct QuestOpsPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl QuestOpsPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl QuestOpsRepository for QuestOpsPostgres {
    async fn add(&self, add_quest_entity: AddQuestEntity) -> Result<i32> {
        unimplemented!()
    }
    async fn edit(&self, quest_id: i32, edit_quest_entity: EditQuestEntity) -> Result<i32> {
        unimplemented!()
    }
    async fn remove(&self, quest_id: i32, guild_commander_id: i32) -> Result<()> {
        unimplemented!()
    }
}

use std::sync::Arc;

use axum::async_trait;

use anyhow::Result;

use crate::{
    domain::{
        entities::adventurers::{AdventurerEntity, RegisterAdventurerEntity},
        repository::adventurers::AdventurersRepository,
        value_objects::adventurer_model::RegisterAdventurerModel,
    },
    infastructure::postgres::postgres_connection::PgPoolSquad,
};

pub struct AdventurerPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl AdventurerPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl AdventurersRepository for AdventurerPostgres {
    /*
    async fn register(&self, register_adventurer_model: RegisterAdventurerModel) -> Result<i32>;
    async fn find_by_username(&self, username: String) -> Result<AdventurerEntity>;
     */

    async fn register(&self, register_adventurer_model: RegisterAdventurerEntity) -> Result<i32> {
        unimplemented!()
    }

    async fn find_by_username(&self, username: String) -> Result<AdventurerEntity> {
        unimplemented!()
    }
}

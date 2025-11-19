use std::sync::Arc;

use axum::async_trait;

use anyhow::{Ok, Result};
use diesel::{dsl::insert_into, prelude::*};

use crate::{
    domain::{
        entities::adventurers::{AdventurerEntity, RegisterAdventurerEntity},
        repository::adventurers::AdventurersRepository,
    },
    infastructure::postgres::{postgres_connection::PgPoolSquad, schema::adventurers},
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
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let result = insert_into(adventurers::table)
            .values(register_adventurer_model)
            .returning(adventurers::id)
            .get_result::<i32>(&mut conn)?;

        Ok(result)
    }

    async fn find_by_username(&self, username: String) -> Result<AdventurerEntity> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let result = adventurers::table
            .filter(adventurers::username.eq(username))
            .select(AdventurerEntity::as_select())
            .first::<AdventurerEntity>(&mut conn)?;

        Ok(result)
    }
}

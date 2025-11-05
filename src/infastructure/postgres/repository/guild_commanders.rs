use anyhow::Result;
use axum::async_trait;
use std::sync::Arc;

use crate::{
    domain::{
        entities::guild_commanders::{GuildCommanderEntity, RegisterGuildCommanderEntity},
        repository::guild_commanders::GuildCommandersRepository,
    },
    infastructure::postgres::postgres_connection::PgPoolSquad,
};

pub struct GuildCommanderPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl GuildCommanderPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl GuildCommandersRepository for GuildCommanderPostgres {
    async fn register(
        &self,
        register_guild_commander_entity: RegisterGuildCommanderEntity,
    ) -> Result<i32> {
        unimplemented!()
    }

    async fn find_by_username(&self, username: String) -> Result<GuildCommanderEntity> {
        unimplemented!()
    }
}

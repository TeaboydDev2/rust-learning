use std::sync::Arc;

use axum::{Json, Router, extract::State, response::IntoResponse, routing::post};

use crate::{
    application::usecase::guild_commanders::GuildCommandersUseCase,
    domain::{
        repository::guild_commanders::GuildCommandersRepository,
        value_objects::guild_commander_model::RegisterGuildCommanderModel,
    },
    infastructure::postgres::{
        postgres_connection::PgPoolSquad, repository::guild_commanders::GuildCommanderPostgres,
    },
};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let guild_commanders_repository = GuildCommanderPostgres::new(db_pool);
    let guild_commanders_use_case =
        GuildCommandersUseCase::new(Arc::new(guild_commanders_repository));

    Router::new()
        .route("/", post(register))
        .with_state(Arc::new(guild_commanders_use_case))
}

pub async fn register<T>(
    State(adventurer_use_case): State<Arc<GuildCommandersUseCase<T>>>,
    Json(register_adventurer_model): Json<RegisterGuildCommanderModel>,
) -> impl IntoResponse
where
    T: GuildCommandersRepository + Send + Sync,
{
}

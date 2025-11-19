use std::sync::Arc;

use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};

use crate::{
    application::usecase::adventurers::AdventurersUseCase,
    domain::{
        repository::adventurers::AdventurersRepository,
        value_objects::adventurer_model::RegisterAdventurerModel,
    },
    infastructure::postgres::{
        postgres_connection::PgPoolSquad, repository::adventurers::AdventurerPostgres,
    },
};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let adventurer_repository = AdventurerPostgres::new(db_pool);
    let adventurer_use_case = AdventurersUseCase::new(Arc::new(adventurer_repository));

    Router::new()
        .route("/", post(register))
        .with_state(Arc::new(adventurer_use_case))
}

pub async fn register<T>(
    State(adventurer_use_case): State<Arc<AdventurersUseCase<T>>>,
    Json(register_adventurer_model): Json<RegisterAdventurerModel>,
) -> impl IntoResponse
where
    T: AdventurersRepository + Send + Sync,
{
    match adventurer_use_case
        .register(register_adventurer_model)
        .await
    {
        Ok(adventurer_id) => (
            StatusCode::CREATED,
            format!("Register adventurer id: {} successfully", adventurer_id),
        )
            .into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

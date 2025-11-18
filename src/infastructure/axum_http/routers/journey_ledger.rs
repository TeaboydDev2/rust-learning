use std::sync::Arc;

use axum::{
    Extension, Router,
    extract::{Path, State},
    response::IntoResponse,
    routing::patch,
};

use crate::{
    application::usecase::journey_ledger::JourneyLedgerUseCase,
    domain::repository::{
        journey_ledger::JourneyLedgerRepository, quest_viewing::QuestViewingRepository,
    },
    infastructure::postgres::{
        postgres_connection::PgPoolSquad,
        repository::{journey_ledger::JourneyLedgerPostgres, quest_viewing::QuestViewingPostgres},
    },
};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let journey_ledger_repository = JourneyLedgerPostgres::new(Arc::clone(&db_pool));
    let quest_viewing_repository = QuestViewingPostgres::new(Arc::clone(&db_pool));
    let journey_ledger_use_case = JourneyLedgerUseCase::new(
        Arc::new(journey_ledger_repository),
        Arc::new(quest_viewing_repository),
    );

    Router::new()
        .route("/in-journey/:quest_id", patch(in_journey))
        .route("/to-completed/:quest_id", patch(to_complete))
        .route("/to-failed/:quest_id", patch(to_failed))
        .with_state(Arc::new(journey_ledger_use_case))
}

pub async fn in_journey<T1, T2>(
    State(journey_ledger_use_case): State<Arc<JourneyLedgerUseCase<T1, T2>>>,
    Extension(adventurer_id): Extension<i32>,
    Path(quest_id): Path<i32>,
) -> impl IntoResponse
where
    T1: JourneyLedgerRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
}

pub async fn to_complete<T1, T2>(
    State(journey_ledger_use_case): State<Arc<JourneyLedgerUseCase<T1, T2>>>,
    Extension(adventurer_id): Extension<i32>,
    Path(quest_id): Path<i32>,
) -> impl IntoResponse
where
    T1: JourneyLedgerRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
}

pub async fn to_failed<T1, T2>(
    State(journey_ledger_use_case): State<Arc<JourneyLedgerUseCase<T1, T2>>>,
    Extension(adventurer_id): Extension<i32>,
    Path(quest_id): Path<i32>,
) -> impl IntoResponse
where
    T1: JourneyLedgerRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
}

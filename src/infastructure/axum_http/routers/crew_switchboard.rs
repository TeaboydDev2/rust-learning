use std::sync::Arc;

use axum::Router;

use crate::infastructure::postgres::postgres_connection::PgPoolSquad;

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    Router::new()
}

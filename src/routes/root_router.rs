use axum::Router;
use crate::AppState;

use super::profiles_router::get_profile_router;

pub async fn get_root_router(state: AppState) -> Router {
    Router::new()
        .merge(get_profile_router(state))
}
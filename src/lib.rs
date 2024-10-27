pub mod controllers {
    pub mod profiles {
        pub mod profile_controller;
    }
    pub mod lib {
        pub mod response;
    }
}
pub mod repository {
    pub mod profiles {
        pub mod profile_repo;
        pub mod profile_model;
    }
}
pub mod routes {
    pub mod root_router;
    pub mod profiles_router;
    pub mod messages_router;
}
pub mod lib {
    pub mod app_state;
}

use lib::app_state::AppState;
use routes::root_router::get_root_router;
use sqlx::postgres::PgPoolOptions;

pub async fn run() {
    let pool = PgPoolOptions::new().max_connections(5).connect("").await;
    let state = AppState { pool: pool.unwrap() };

    let router = get_root_router(state).await;
    
    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap(),
        router
    )
    .await
    .unwrap();        
}

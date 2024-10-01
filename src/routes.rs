use axum::{
    routing::{get, post},
    Router,
};
use sqlx::SqlitePool;

use crate::handlers::{
    get_profile, get_progress, get_study_sessions, get_subjects, get_tasks, login_user,
    register_user,
};

pub fn app_router(pool: SqlitePool) -> Router {
    Router::new()
        .nest(
            "/api",
            Router::new()
                .route("/register", post(register_user))
                .route("/login", post(login_user))
                .route("/progress", get(get_progress))
                .route("/subjects", get(get_subjects))
                .route("/profile", get(get_profile))
                .route("/tasks", get(get_tasks))
                .route("/study-sessions", get(get_study_sessions)),
        )
        .layer(axum::Extension(pool))
}

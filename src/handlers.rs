// src/handlers.rs

use axum::{extract::Extension, response::IntoResponse, Json};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::SqlitePool;
use std::env;
use uuid::Uuid;
use validator::Validate;

use crate::models::User;


pub async fn register_user(
    Extension(pool): Extension<SqlitePool>,
    Json(payload): Json<RegisterPayload>,
) -> Result<Json<RegisterResponse>, AppError> {

    payload
        .validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;


    let existing_user = sqlx::query_as::<_, User>(
        "SELECT id, username, email, password_hash, created_at FROM users WHERE username = ? OR email = ?",
    )
    .bind(&payload.username)
    .bind(&payload.email)
    .fetch_optional(&pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    if existing_user.is_some() {
        return Err(AppError::ValidationError(
            "Username or email already exists.".into(),
        ));
    }

 
    let user_id = Uuid::new_v4().to_string();

  
    let password_hash =
        hash(&payload.password, DEFAULT_COST).map_err(|e| AppError::HashingError(e.to_string()))?;

 
    let created_at = Utc::now().to_rfc3339();

 
    sqlx::query(
        r#"
        INSERT INTO users (id, username, email, password_hash, created_at)
        VALUES (?, ?, ?, ?, ?)
        "#,
    )
    .bind(&user_id)
    .bind(&payload.username)
    .bind(&payload.email)
    .bind(&password_hash)
    .bind(&created_at)
    .execute(&pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    let token = create_jwt(&user_id)?;

    // todo 
    let response = RegisterResponse {
        id: user_id,
        username: payload.username,
        email: payload.email,
        token,
        created_at,
    };

    Ok(Json(response))
}


#[derive(Debug, Deserialize, Validate)]
pub struct RegisterPayload {
    #[validate(length(min = 3, message = "Username must be at least 3 characters long"))]
    pub username: String,

    #[validate(email(message = "Invalid email address"))]
    pub email: String,

    #[validate(length(min = 6, message = "Password must be at least 6 characters long"))]
    pub password: String,
}

/// Response structure after registering a user.
#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub id: String,
    pub username: String,
    pub email: String,
    pub token: String,
    pub created_at: String,
}


pub async fn login_user(
    Extension(pool): Extension<SqlitePool>,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<LoginResponse>, AppError> {

    payload
        .validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let user = sqlx::query_as::<_, User>(
        "SELECT id, username, email, password_hash, created_at FROM users WHERE username = ?",
    )
    .bind(&payload.username)
    .fetch_one(&pool)
    .await
    .map_err(|_| AppError::AuthenticationError("Invalid username or password".into()))?;

    let is_valid = verify(&payload.password, &user.password_hash)
        .map_err(|e| AppError::HashingError(e.to_string()))?;

    if !is_valid {
        return Err(AppError::AuthenticationError(
            "Invalid username or password".into(),
        ));
    }

    let token = create_jwt(&user.id)?;

    let response = LoginResponse { token };

    Ok(Json(response))
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginPayload {
    #[validate(length(min = 3, message = "Username must be at least 3 characters long"))]
    pub username: String,

    #[validate(length(min = 6, message = "Password must be at least 6 characters long"))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
}

fn create_jwt(user_id: &str) -> Result<String, AppError> {
    #[derive(Debug, Serialize, Deserialize)]
    struct Claims {
        sub: String, 
        exp: usize,  // todo : change this in future
    }

  
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .ok_or_else(|| AppError::AuthenticationError("Invalid expiration time".into()))?
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration,
    };

    let secret = env::var("JWT_SECRET")
        .map_err(|_| AppError::AuthenticationError("JWT_SECRET not set".into()))?;

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AppError::AuthenticationError(e.to_string()))?;

    Ok(token)
}


#[derive(Debug)]
pub enum AppError {
    DatabaseError(String),
    ValidationError(String),
    HashingError(String),
    AuthenticationError(String),
   // better and more
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::DatabaseError(_) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Internal server error" })),
            )
                .into_response(),
            AppError::ValidationError(msg) => (
                axum::http::StatusCode::BAD_REQUEST,
                Json(json!({ "error": msg })),
            )
                .into_response(),
            AppError::HashingError(_) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Password processing failed" })),
            )
                .into_response(),
            AppError::AuthenticationError(msg) => (
                axum::http::StatusCode::UNAUTHORIZED,
                Json(json!({ "error": msg })),
            )
                .into_response(),
        }
    }
}

pub async fn get_progress(
    Extension(pool): Extension<SqlitePool>,

) -> Result<Json<ProgressResponse>, AppError> {
   

    let progress = sqlx::query_as::<_, Progress>(
        "SELECT completed_tasks, total_tasks FROM progress WHERE user_id = ?",
    )
    .bind("user_id_here") 
    .fetch_one(&pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    let response = ProgressResponse {
        completed_tasks: progress.completed_tasks,
        total_tasks: progress.total_tasks,
    };

    Ok(Json(response))
}
use sqlx::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Progress {
    pub completed_tasks: i32,
    pub total_tasks: i32,
}
#[derive(Debug, Serialize)]
pub struct ProgressResponse {
    pub completed_tasks: i32,
    pub total_tasks: i32,
}

pub async fn get_subjects(
    Extension(pool): Extension<SqlitePool>,

) -> Result<Json<Vec<SubjectResponse>>, AppError> {
    
    let subjects = sqlx::query_as::<_, Subject>(
        "SELECT id, name, description FROM subjects WHERE user_id = ?",
    )
    .bind("user_id_here")
    .fetch_all(&pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    let response: Vec<SubjectResponse> = subjects
        .into_iter()
        .map(|s| SubjectResponse {
            id: s.id,
            name: s.name,
            description: s.description,
        })
        .collect();

    Ok(Json(response))
}

#[derive(Debug, Deserialize, FromRow)]
pub struct Subject {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SubjectResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}

pub async fn get_profile(
    Extension(pool): Extension<SqlitePool>,

) -> Result<Json<ProfileResponse>, AppError> {
    let user =
        sqlx::query_as::<_, User>("SELECT id, username, email, created_at FROM users WHERE id = ?")
            .bind("user_id_here") 
            .fetch_one(&pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    let response = ProfileResponse {
        id: user.id,
        username: user.username,
        email: user.email,
        created_at: user.created_at,
    };

    Ok(Json(response))
}

#[derive(Debug, Serialize)]
pub struct ProfileResponse {
    pub id: String,
    pub username: String,
    pub email: String,
    pub created_at: String,
}

pub async fn get_tasks(
    Extension(pool): Extension<SqlitePool>,
) -> Result<Json<Vec<TaskResponse>>, AppError> {
    let tasks = sqlx::query_as::<_, Task>(
        "SELECT id, title, description, deadline, difficulty_level FROM tasks WHERE subject_id = ?",
    )
    .bind("subject_id_here") 
    .fetch_all(&pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    let response: Vec<TaskResponse> = tasks
        .into_iter()
        .map(|t| TaskResponse {
            id: t.id,
            title: t.title,
            description: t.description,
            deadline: t.deadline,
            difficulty_level: t.difficulty_level,
        })
        .collect();

    Ok(Json(response))
}

#[derive(Debug, Deserialize, FromRow)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub deadline: Option<String>,
    pub difficulty_level: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct TaskResponse {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub deadline: Option<String>,
    pub difficulty_level: Option<i32>,
}

pub async fn get_study_sessions(
    Extension(pool): Extension<SqlitePool>,
) -> Result<Json<Vec<StudySessionResponse>>, AppError> {
    let sessions = sqlx::query_as::<_, StudySession>(
        "SELECT id, scheduled_at, duration, completed FROM study_sessions WHERE task_id = ?",
    )
    .bind("task_id_here") 
    .fetch_all(&pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    let response: Vec<StudySessionResponse> = sessions
        .into_iter()
        .map(|s| StudySessionResponse {
            id: s.id,
            scheduled_at: s.scheduled_at,
            duration: s.duration,
            completed: s.completed,
        })
        .collect();

    Ok(Json(response))
}

#[derive(Debug, Deserialize, FromRow)]
pub struct StudySession {
    pub id: String,
    pub scheduled_at: String,
    pub duration: i32,
    pub completed: bool,
}

#[derive(Debug, Serialize)]
pub struct StudySessionResponse {
    pub id: String,
    pub scheduled_at: String,
    pub duration: i32,
    pub completed: bool,
}

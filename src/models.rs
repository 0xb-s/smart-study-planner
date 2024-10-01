use serde::{Deserialize, Serialize};

/// Represents a user in the system.
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: String, // Using String instead of DateTime
}

/// Represents a subject.
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Subject {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: String, // Using String instead of DateTime
}


#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Task {
    pub id: String,
    pub subject_id: String,
    pub title: String,
    pub description: Option<String>,
    pub deadline: Option<String>, 
    pub difficulty_level: Option<i32>,
    pub created_at: String, // todo better here
}

/// Represents a study session.
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct StudySession {
    pub id: String,
    pub task_id: String,
    pub scheduled_at: String, 
    pub duration: i32,       
    pub completed: bool,
    pub created_at: String, // todo better here
}

/// Represents progress.
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Progress {
    pub id: String,
    pub user_id: String,
    pub subject_id: String,
    pub completed_tasks: i32,
    pub total_tasks: i32,
    pub created_at: String, // todo better here, datatime would be better 
}

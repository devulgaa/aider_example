use hyper::{Body, Request, Response};
use serde_json::json;
use sqlx::PgPool;

use crate::models;

/// Handles creating a new user.
pub async fn create_user(
    pool: &PgPool,
    username: String,
    password: String,
    birthdate: Option<String>,
) -> Result<Response<Body>, Box<dyn std::error::Error>> {
    let query = "INSERT INTO users (username, password, birthdate) VALUES ($1, $2, $3) RETURNING *";
    let user = sqlx::query_as::<_, models::user::User>(query)
        .bind(username)
        .bind(password)
        .bind(birthdate)
        .fetch_one(pool)
        .await?;

    Ok(Response::new(Body::from(serde_json::to_string(&user)?)))
}

/// Handles retrieving all users.
pub async fn get_users(pool: &PgPool) -> Result<Response<Body>, Box<dyn std::error::Error>> {
    let query = "SELECT * FROM users";
    let users = sqlx::query_as::<_, models::User>(query)
        .fetch_all(pool)
        .await?;

    Ok(Response::new(Body::from(serde_json::to_string(&users)?)))
}

/// Handles retrieving a specific user by ID.
pub async fn get_user_by_id(
    pool: &PgPool,
    id: i32,
) -> Result<Response<Body>, Box<dyn std::error::Error>> {
    let query = "SELECT * FROM users WHERE id = $1";
    let user = sqlx::query_as::<_, models::User>(query)
        .bind(id)
        .fetch_one(pool)
        .await?;

    Ok(Response::new(Body::from(serde_json::to_string(&user)?)))
}

/// Handles updating an existing user.
pub async fn update_user(
    pool: &PgPool,
    id: i32,
    username: String,
    password: String,
    birthdate: Option<String>,
) -> Result<Response<Body>, Box<dyn std::error::Error>> {
    let query =
        "UPDATE users SET username = $1, password = $2, birthdate = $3 WHERE id = $4 RETURNING *";
    let user = sqlx::query_as::<_, models::User>(query)
        .bind(username)
        .bind(password)
        .bind(birthdate)
        .bind(id)
        .fetch_one(pool)
        .await?;

    Ok(Response::new(Body::from(serde_json::to_string(&user)?)))
}

/// Handles deleting a user.
pub async fn delete_user(
    pool: &PgPool,
    id: i32,
) -> Result<Response<Body>, Box<dyn std::error::Error>> {
    let query = "DELETE FROM users WHERE id = $1";
    sqlx::query(query).bind(id).execute(pool).await?;

    Ok(Response::new(Body::from(serde_json::to_string(
        &json!({ "message": "User deleted" }),
    )?) as Body))
}

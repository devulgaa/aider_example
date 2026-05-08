use sqlx::PgPool;

/// Inserts a new user into the database.
pub async fn insert_user(pool: &PgPool, username: String, password: String, birthdate: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let query = "INSERT INTO users (username, password, birthdate) VALUES ($1, $2, $3)";
    sqlx::query(query)
        .bind(username)
        .bind(password)
        .bind(birthdate)
        .execute(pool)
        .await?;

    Ok(())
}

/// Deletes a user from the database by ID.
pub async fn delete_user(pool: &PgPool, id: i32) -> Result<(), Box<dyn std::error::Error>> {
    let query = "DELETE FROM users WHERE id = $1";
    sqlx::query(query)
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}

use axum::{routing::get, Router, Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, FromRow, sqlite::SqlitePoolOptions};

#[derive(Serialize, FromRow)]
struct User {
    id: i64,
    name: String,
}

#[derive(Deserialize)]
struct NewUser {
    name: String,
}

async fn list_users(State(pool): State<SqlitePool>) -> Result<Json<Vec<User>>, StatusCode> {
    let users = sqlx::query_as::<_, User>("SELECT id, name FROM users")
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(users))
}

async fn create_user(State(pool): State<SqlitePool>, Json(payload): Json<NewUser>) -> Result<Json<User>, StatusCode> {
    let user = sqlx::query_as::<_, User>("INSERT INTO users (name) VALUES (?) RETURNING id, name")
        .bind(payload.name)
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(user))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // initialize connection pool
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite:database.db").await?;

    // ensure table exists
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL)"
    )
    .execute(&pool)
    .await?;

    // build our application with some routes
    let app = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .route("/users", get(list_users).post(create_user))
        .with_state(pool.clone());

    println!("Listening on http://127.0.0.1:3000");
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}

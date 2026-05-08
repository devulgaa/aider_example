#![warn(missing_docs)]

// A simple Rust crate for a CRUD application.
pub mod handlers;
pub mod models;

use hyper::{Body, Request, Response};
use hyper::service::{make_service_fn, service_fn};
use hyper::Server;
use sqlx::PgPool;

/// Initializes the database connection pool.
pub async fn init_db() -> Result<PgPool, Box<dyn std::error::Error>> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Ok(sqlx::PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?)
}

/// Starts the HTTP server.
pub async fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    use hyper::{Body, Request, Response, Server};
    use hyper::service::{make_service_fn, service_fn};

    let addr = ([127, 0, 0, 1], 3000).into();

    let make_svc = make_service_fn(|_conn| async { Ok::<_, hyper::Error>(service_fn(handler)) });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }

    Ok(())
}

/// Handles incoming HTTP requests.
async fn handler(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    // Implement your routing logic here
    Ok(Response::new(Body::from("Hello, world!")))
}

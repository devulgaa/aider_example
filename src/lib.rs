#![warn(missing_docs)]

/// A simple Rust crate for a CRUD application.
pub mod handlers;
pub mod models;

/// Initializes the database connection pool.
pub async fn init_db() -> sqlx::PgPool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    sqlx::PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to the database")
}

/// Starts the HTTP server.
pub async fn start_server() {
    use hyper::{Body, Request, Response, Server};
    use hyper::service::{make_service_fn, service_fn};

    let addr = ([127, 0, 0, 1], 3000).into();

    let make_svc = make_service_fn(|_conn| {
        async { Ok::<_, hyper::Error>(service_fn(handler)) }
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}

/// Handles incoming HTTP requests.
async fn handler(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    // Implement your routing logic here
    Ok(Response::new(Body::from("Hello, world!")))
}

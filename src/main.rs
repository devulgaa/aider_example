mod lib;

#[tokio::main]
async fn main() {
    if let Err(e) = lib::init_db().await {
        eprintln!("Failed to initialize database: {}", e);
        return;
    }

    if let Err(e) = lib::start_server().await {
        eprintln!("Server error: {}", e);
    }
}

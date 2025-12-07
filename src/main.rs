use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about = "High-performance S3-compatible storage in Rust")]
struct Args {
    #[arg(long, default_value = "0.0.0.0:9000")]
    listen: String,

    #[arg(long, default_value = "./data")]
    data_dir: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let args = Args::parse();
    println!("RustFS Storage v{} starting...", env!("CARGO_PKG_VERSION"));
    println!("Listening on http://{}", args.listen);
    println!("Data directory: {}", args.data_dir);

    let app = rust_s3_storage::server::router(args.data_dir);
    let listener = tokio::net::TcpListener::bind(&args.listen).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

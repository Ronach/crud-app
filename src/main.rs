mod handlers;
use axum::http;
use axum::routing::{get, post, Router};
/*
works with postgres and tokio which is the asynchronous runtime for axum
tokio est un framework d'exécution asynchrone populaire pour Rust. 
Vous pouvez utiliser d'autres frameworks tels que async-std en fonction de vos besoins.
*/
use sqlx::postgres::PgPoolOptions;
use std::env;


// L'attribut #[tokio::main] est utilisé dans Rust pour indiquer que la fonction main est une fonction asynchrone qui utilise le runtime de l'exécution asynchrone de Tokio.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let port: i32 = 3000;
    let port: String = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr: String = format!("0.0.0.0:{}", port);
    
    let database_url = env::var("DATABASE_URL").expect("missing DATABASE_URL env");

    let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(&database_url)
    .await?;

    let app: Router = Router::new()
    .route("/",get(handlers::health))
    .route("/quotes", post(handlers::create_quote));

    axum::Server::bind(&addr.parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();

    Ok(())
}
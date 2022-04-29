use actix_web::{get, web, App, HttpServer, Result};
use rand::prelude::*;

#[get("/{max}")]
async fn index(path: web::Path<isize>) -> Result<String> {
    let max = path.into_inner();
    let mut rng = rand::thread_rng();
    let i = rng.gen_range(0..max);
    Ok(format!("{}..{} => {}", 0, max, i))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

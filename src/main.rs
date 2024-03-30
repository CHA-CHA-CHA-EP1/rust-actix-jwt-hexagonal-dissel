#![allow(unused_imports)]
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use rust_jwt_auth::*;
use dotenvy::dotenv;
use std::env;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    let user_service = Box::new(services::user_service::UserServiceImpl::new());
    let user_controller = controllers::user_controller::UserController::new(user_service);

    println!("Server is stringing on port 8080");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(user_controller.clone()))
            .configure(controllers::user_controller::config)
            .service(hello)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}


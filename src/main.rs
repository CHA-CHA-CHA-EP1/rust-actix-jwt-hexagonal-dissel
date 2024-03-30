#![allow(unused_imports)]
use std::thread;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection
};

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
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("database error");

    let user_repository = Box::new(repositories::userrepo::user_repository::UserRepositoryImpl::new(pool.clone()));
    let user_service = Box::new(services::user_service::UserServiceImpl::new(user_repository));
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


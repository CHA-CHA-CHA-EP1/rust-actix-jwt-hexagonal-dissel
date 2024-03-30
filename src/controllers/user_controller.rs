use actix_web::{Responder, HttpResponse, web, post};
use crate::{domain::user::NewUser, services::user_service::UserService};

#[derive(Clone)]
pub struct UserController {
    user_service: Box<dyn UserService>
}

impl UserController {
    pub fn new(user_service: Box<dyn UserService>) -> Self {
        Self {
            user_service
        }
    }
}

impl UserController {
    pub async fn create_user(&self, user_data: web::Json<NewUser>) -> impl Responder {
        println!("{:?}", user_data);
        HttpResponse::Ok().body("Creating user")
    }
}

#[post("/signup")]
async fn signup(user_controller: web::Data<UserController>, user_data: web::Json<NewUser>) -> impl Responder {
    user_controller.create_user(user_data).await
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth") 
            .service(signup)
    );
}

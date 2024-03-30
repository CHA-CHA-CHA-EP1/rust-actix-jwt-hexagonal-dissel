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
        let user_data = user_data.into_inner();
        match self.user_service.create_user(user_data) {
            Ok(_) => return HttpResponse::Created().body("Creating user"),
            Err(_) => return HttpResponse::InternalServerError().body("Error creating user")
        }
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

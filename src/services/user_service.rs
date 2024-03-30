use crate::domain::user::NewUser;
use trait_async::trait_async;
use crate::repositories::userrepo::user_repository::UserRepository;

use dyn_clone::DynClone;

#[trait_async]
pub trait UserService: DynClone + Send {
    fn create_user(&self, user_data: NewUser) -> Result<NewUser, String>;
}

dyn_clone::clone_trait_object!(UserService);

#[derive(Clone)]
pub struct UserServiceImpl {
    us_repo: Box<dyn UserRepository>
}

impl UserServiceImpl {
    pub fn new(
        us_repo: Box<dyn UserRepository>
        ) -> Self {
        UserServiceImpl {
            us_repo
        }
    }
}

impl UserService for UserServiceImpl {
    fn create_user(&self, mut user_data: NewUser) -> Result<NewUser, String> {
        let hashed_password = crate::utils::hash::hashData(user_data.password.as_str());
        user_data.password = hashed_password;
        let user_response = self.us_repo.save(user_data);
        match user_response {
            Ok(user) => Ok(user),
            Err(_) => Err("Error creating user".to_string())
        }
    }
}

use crate::domain::user::NewUser;
use trait_async::trait_async;

use dyn_clone::DynClone;

#[trait_async]
pub trait UserService: DynClone + Send + Sync {
    fn create_user(&self, user_data: NewUser) -> Result<(), String>;
}

dyn_clone::clone_trait_object!(UserService);

#[derive(Clone)]
pub struct UserServiceImpl {}

impl UserServiceImpl {
    pub fn new() -> Self {
        UserServiceImpl {}
    }
}

impl UserService for UserServiceImpl {
    fn create_user(&self, user_data: NewUser) -> Result<(), String> {
        println!("{:?}", user_data);
        Ok(())
    }
}

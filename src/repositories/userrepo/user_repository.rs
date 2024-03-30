use crate::domain::user::User;

trait UserRepository {
    fn find_by_id(&self, id: u64) -> Option<User>;
    fn find_by_username(&self, username: &str) -> Option<User>;
    fn save(&self, user: User) -> Result<(), String>;
    fn delete(&self, id: u64) -> Result<(), String>;
}

pub struct UserRepositoryImpl {
    users: Vec<User>
}

impl UserRepositoryImpl {
    pub fn new() -> Self {
        Self {
            users: vec![]
        }
    }
}

impl UserRepository for UserRepositoryImpl {
    fn find_by_id(&self, id: u64) -> Option<User> {
        self.users.iter().find(|user| user.id == id).cloned()
    }

    fn find_by_username(&self, username: &str) -> Option<User> {
        self.users.iter().find(|user| user.username == username).cloned()
    }

    fn save(&self, user: User) -> Result<(), String> {
        if self.users.iter().any(|u| u.id == user.id) {
            return Err("User already exists".to_string());
        }

        if self.users.iter().any(|u| u.username == user.username) {
            return Err("Username already exists".to_string());
        }

        Ok(())
    }

    fn delete(&self, id: u64) -> Result<(), String> {
        if self.users.iter().any(|u| u.id == id) {
            return Err("User not found".to_string());
        }

        Ok(())
    }
}

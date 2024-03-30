use diesel::prelude::*;
use serde::Deserialize;

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub username: String,
    pub password: String
}

#[derive(Insertable, Debug, Deserialize)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub name: String,
    pub username: String,
    pub password: String
}

impl User {
    pub fn new(id: u64, name: String, username: String, password: String) -> Self {
        User {
            id,
            name,
            username,
            password
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_user() {
        let user = User::new(1, "John Doe".to_string(), "johndoe".to_string(), "password".to_string());
        assert_eq!(user.id, 1);
        assert_eq!(user.name, "John Doe");
        assert_eq!(user.username, "johndoe");
        assert_eq!(user.password, "password");
    }
}


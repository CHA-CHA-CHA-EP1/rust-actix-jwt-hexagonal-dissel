use trait_async::trait_async;
use dyn_clone::DynClone;
use crate::domain::user::NewUser;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection, RunQueryDsl
};

#[trait_async]
pub trait UserRepository: DynClone + Send {
    fn save(&self, user_data: NewUser) -> Result<NewUser, String>;
}

dyn_clone::clone_trait_object!(UserRepository);


#[derive(Clone)]
pub struct UserRepositoryImpl {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl UserRepositoryImpl {
    pub fn new(
            pool: Pool<ConnectionManager<PgConnection>>,
        ) -> Self {
        Self {
            pool,
        }
    }
}

impl UserRepository for UserRepositoryImpl {
    fn save(&self, user_data: NewUser) -> Result<NewUser, String> {
        let mut conn = self.pool.get().unwrap();
        use crate::schema::users;

        let results = diesel::insert_into(users::table)
            .values(&user_data)
            .execute(&mut conn);
        
        match results {
            Ok(_) => println!("User created"),
            Err(e) => println!("Error creating user: {:?}", e),
        }

        println!("Creating user");
        Ok(user_data)
    }
}

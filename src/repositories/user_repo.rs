use crate::{errors::AppError, models::User, schema::users};
use diesel::{prelude::*, MysqlConnection};

pub struct UserRepository {
    conn: MysqlConnection,
}

impl UserRepository {
    pub fn new(conn: MysqlConnection) -> Self {
        Self { conn }
    }

    pub fn find_by_id(&mut self, user_id: &str) -> Result<User, AppError> {
        use crate::schema::users::dsl::*;
        users
            .find(user_id)
            .first(&mut self.conn)
            .map_err(AppError::Database)
    }

    pub fn find_by_email(&mut self, user_email: &str) -> Result<Option<User>, AppError> {
        use crate::schema::users::dsl::*;
        users
            .filter(email.eq(user_email))
            .first::<User>(&mut self.conn)
            .optional()
            .map_err(AppError::Database)
    }

    pub fn create(&mut self, user: &User) -> Result<User, AppError> {
        use crate::schema::users::dsl::*;
        diesel::insert_into(users)
            .values(user)
            .execute(&mut self.conn)
            .map_err(AppError::Database)?;

        Ok(user.clone())
    }
}

use sqlx::{query_as, SqlitePool};

use crate::common::AppError;
use crate::scheme::{InsertUser, User};

pub struct UserRepo;

impl UserRepo {
    pub async fn get_all_users(pool: &SqlitePool) -> Result<Vec<User>, AppError> {
        let users = query_as::<_, User>("SELECT * FROM user_table")
            .fetch_all(pool)
            .await?;
        Ok(users)
    }

    pub async fn get_user_by_id(pool: &SqlitePool, id: i32) -> Result<User, AppError> {
        let user = query_as::<_, User>(&format!("SELECT * FROM user_table WHERE id = {}", id))
            .fetch_one(pool)
            .await?;
        Ok(user)
    }

    pub async fn get_user_by_username(pool: &SqlitePool, username: &str) -> Result<User, AppError> {
        let user = query_as::<_, User>(&format!(
            "SELECT * FROM user_table WHERE username = '{}'",
            username
        ))
        .fetch_one(pool)
        .await?;
        Ok(user)
    }

    pub async fn insert_user(pool: &SqlitePool, insert_user: InsertUser) -> Result<User, AppError> {
        let user_id = query_as::<_, (i32,)>(&format!(
            "INSERT INTO user_table (username, password_hash, is_superuser) VALUES ('{}', '{}', '{}') RETURNING id",
            insert_user.username,
            insert_user.password_hash,
            insert_user.is_superuser
        ))
        .fetch_one(pool)
        .await?.0;
        Ok(User {
            id: user_id,
            username: insert_user.username,
            password_hash: insert_user.password_hash,
            is_superuser: insert_user.is_superuser,
        })
    }
}

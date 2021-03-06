use poem::web::Data;
use poem_openapi::{param::Path, payload::Json, OpenApi};
use sqlx::SqlitePool;

use super::Tag;
use crate::db::repo::UserRepo;
use crate::response::{
    GetAllResponseError, GetAllResponseSuccess, GetOneResponseError, GetOneResponseSuccess,
    PostResponseError, PostResponseSuccess,
};
use crate::scheme::{CreateUser, InsertUser, User};
use crate::service::auth_service::{superuser_scope, AuthService, Claims};

pub struct UserController;

#[OpenApi(prefix_path = "/user", tag = "Tag::User")]
impl UserController {
    /// Get all users.
    #[oai(path = "/", method = "get")]
    async fn get_all_users(
        &self,
        pool: Data<&SqlitePool>,
        claims: Claims,
    ) -> Result<GetAllResponseSuccess<User>, GetAllResponseError> {
        superuser_scope(claims)?;
        let users = UserRepo::get_all_users(pool.0).await?;
        let resp = GetAllResponseSuccess::new(users);
        Ok(resp)
    }

    /// Get user by ID.
    #[oai(path = "/:id", method = "get")]
    async fn get_user_by_id(
        &self,
        pool: Data<&SqlitePool>,
        id: Path<i32>,
        claims: Claims,
    ) -> Result<GetOneResponseSuccess<User>, GetOneResponseError> {
        superuser_scope(claims)?;
        let user = UserRepo::get_user_by_id(pool.0, id.0).await?;
        let resp = GetOneResponseSuccess::new(user);
        Ok(resp)
    }

    /// Create new user.
    #[oai(path = "/", method = "post")]
    async fn create_user(
        &self,
        pool: Data<&SqlitePool>,
        user: Json<CreateUser>,
        claims: Claims,
    ) -> Result<PostResponseSuccess<User>, PostResponseError> {
        superuser_scope(claims)?;
        let insert_user = InsertUser {
            username: user.0.username,
            password_hash: AuthService::hash_password(&user.0.password)?,
            is_superuser: 0,
        };
        let user = UserRepo::insert_user(pool.0, insert_user).await?;
        let resp = PostResponseSuccess::new(user);
        Ok(resp)
    }
}

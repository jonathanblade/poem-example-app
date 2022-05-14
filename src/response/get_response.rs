use poem_openapi::{
    payload::Json,
    types::{ParseFromJSON, ToJSON},
    ApiResponse,
};

use super::{ResponseError, ResponseSuccess};
use crate::common::AppError;

#[derive(ApiResponse)]
pub enum GetOneResponseSuccess<T: ParseFromJSON + ToJSON + Send + Sync> {
    /// Request completed successfully.
    #[oai(status = 200)]
    OK(Json<ResponseSuccess<T>>),
}

impl<T: ParseFromJSON + ToJSON + Send + Sync> GetOneResponseSuccess<T> {
    pub fn new(result: T) -> Self {
        GetOneResponseSuccess::OK(Json(ResponseSuccess::new(result)))
    }
}

#[derive(ApiResponse)]
pub enum GetOneResponseError {
    /// Bad request.
    #[oai(status = 400)]
    BadRequest(Json<ResponseError>),
    /// Authorization failed.
    #[oai(status = 401)]
    Unauthorized(Json<ResponseError>),
    /// Object not found.
    #[oai(status = 404)]
    NotFound(Json<ResponseError>),
    /// Internal server error.
    #[oai(status = 500)]
    InternalError(Json<ResponseError>),
}

impl From<AppError> for GetOneResponseError {
    fn from(e: AppError) -> Self {
        match e {
            // 400
            AppError::BadRequest(_)
            | AppError::MethodNotAllowed
            | AppError::MissingContentType
            | AppError::UnsupportedContentType(_)
            | AppError::MalformedRequestPayload => GetOneResponseError::BadRequest(Json(e.into())),
            // 401
            AppError::InvalidAccessToken
            | AppError::MissingAccessToken
            | AppError::AccessTokenExpired
            | AppError::InvalidCredentials
            | AppError::SuperuserScopeRequired => GetOneResponseError::Unauthorized(Json(e.into())),
            // 404
            AppError::ObjectNotFound | AppError::ResourceNotFound(_) => {
                GetOneResponseError::NotFound(Json(e.into()))
            }
            // 500
            _ => GetOneResponseError::InternalError(Json(e.into())),
        }
    }
}

#[derive(ApiResponse)]
pub enum GetAllResponseSuccess<T: ParseFromJSON + ToJSON + Send + Sync> {
    /// Request completed successfully.
    #[oai(status = 200)]
    OK(Json<ResponseSuccess<Vec<T>>>),
}

impl<T: ParseFromJSON + ToJSON + Send + Sync> GetAllResponseSuccess<T> {
    pub fn new(result: Vec<T>) -> Self {
        GetAllResponseSuccess::OK(Json(ResponseSuccess::new(result)))
    }
}

#[derive(ApiResponse)]
pub enum GetAllResponseError {
    /// Bad request.
    #[oai(status = 400)]
    BadRequest(Json<ResponseError>),
    /// Authorization failed.
    #[oai(status = 401)]
    Unauthorized(Json<ResponseError>),
    /// Internal server error.
    #[oai(status = 500)]
    InternalError(Json<ResponseError>),
}

impl From<AppError> for GetAllResponseError {
    fn from(e: AppError) -> Self {
        match e {
            // 400
            AppError::BadRequest(_)
            | AppError::MethodNotAllowed
            | AppError::MissingContentType
            | AppError::UnsupportedContentType(_)
            | AppError::MalformedRequestPayload => GetAllResponseError::BadRequest(Json(e.into())),
            // 401
            AppError::InvalidAccessToken
            | AppError::MissingAccessToken
            | AppError::AccessTokenExpired
            | AppError::InvalidCredentials
            | AppError::SuperuserScopeRequired => GetAllResponseError::Unauthorized(Json(e.into())),
            // 500
            _ => GetAllResponseError::InternalError(Json(e.into())),
        }
    }
}

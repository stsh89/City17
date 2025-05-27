use crate::operations::OperationalError;
use rocket::serde::json::Json;

#[derive(Debug, Responder)]
pub enum ApiError {
    #[response(status = 400, content_type = "json")]
    BadRequest(Json<BadRequestError>),

    #[response(status = 500, content_type = "json")]
    Internal(Json<InternalError>),
}

#[derive(Debug, rocket::serde::Serialize)]
#[serde(crate = "rocket::serde")]
pub struct InternalError {
    error: String,
}

#[derive(Debug, rocket::serde::Serialize)]
#[serde(crate = "rocket::serde")]
pub struct BadRequestError {
    error: String,
}

impl From<OperationalError> for ApiError {
    fn from(value: OperationalError) -> Self {
        match value {
            OperationalError::Internal(report) => ApiError::Internal(Json(InternalError {
                error: report.to_string(),
            })),
            OperationalError::Repo(error) => ApiError::Internal(Json(InternalError {
                error: error.to_string(),
            })),
            OperationalError::InvalidArgument(error) => {
                ApiError::BadRequest(Json(BadRequestError { error }))
            }
            OperationalError::Validation(error) => {
                ApiError::BadRequest(Json(BadRequestError { error }))
            }
        }
    }
}

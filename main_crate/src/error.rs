use std::fmt::Display;

use derive_more::{derive, Display};
use serde_json::json;

use crate::queries::{self, QueryErr};

use actix_web::{error, http::{header::ContentType, StatusCode}, HttpResponse, Responder};




#[derive(Debug, Display)]
pub enum ServiceError{
    #[display("Database Query")]
    DatabaseQuery(QueryErr),

    #[display("Invalid Cookie")]
    CookieInternals,

    #[display("Bad Request")]
    BadClientData,

    #[display("Internal Error")]
    InternalErr(String)
}

impl From<QueryErr> for ServiceError {
    fn from(value: QueryErr) -> Self {
        ServiceError::DatabaseQuery(value)
    }
}

impl error::ResponseError for ServiceError{
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(json!({
                "error": {
                    "message": match self {
                        ServiceError::DatabaseQuery(response_error) => {
                            format!("Database Query Error: {}", response_error)
                        },
                        ServiceError::CookieInternals => {
                            "Cookie Internal Error: {}".to_string()
                        },
                        ServiceError::BadClientData => {
                            "Please ensure all fields use the correct data types 
                            and respect the defined value limits.".to_string()
                        },
                        ServiceError::InternalErr(msg) => {
                            format!("Internal Error: {}", msg)
                        },
                    }
                }
            }))
    }


    fn status_code(&self) -> StatusCode {
        match *self {
            ServiceError::DatabaseQuery(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::CookieInternals => StatusCode::UNAUTHORIZED,
            ServiceError::BadClientData => StatusCode::BAD_REQUEST,
            ServiceError::InternalErr(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

}



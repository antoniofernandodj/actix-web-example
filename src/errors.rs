use actix_web::{error, http::StatusCode, HttpResponse};
use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
}

#[derive(Debug, Display, Error)]
#[display(fmt = "Bad Request: {}", name)]
pub struct HTTP400Error {
    pub name: String,
}

impl error::ResponseError for HTTP400Error {
    fn status_code(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }

    fn error_response(&self) -> HttpResponse {
        let error_response = ErrorResponse {
            error: "Bad Request".to_string(),
            message: self.name.clone(),
        };
        HttpResponse::BadRequest().json(error_response)
    }
}

#[derive(Debug, Display, Error)]
#[display(fmt = "Unauthorized: {}", name)]
pub struct HTTP401Error {
    pub name: String,
}

impl error::ResponseError for HTTP401Error {
    fn status_code(&self) -> StatusCode {
        StatusCode::UNAUTHORIZED
    }

    fn error_response(&self) -> HttpResponse {
        let error_response = ErrorResponse {
            error: "Unauthorized".to_string(),
            message: self.name.clone(),
        };
        HttpResponse::Unauthorized().json(error_response)
    }
}

#[derive(Debug, Display, Error)]
#[display(fmt = "Forbidden: {}", name)]
pub struct HTTP403Error {
    pub name: String,
}

impl error::ResponseError for HTTP403Error {
    fn status_code(&self) -> StatusCode {
        StatusCode::FORBIDDEN
    }

    fn error_response(&self) -> HttpResponse {
        let error_response = ErrorResponse {
            error: "Forbidden".to_string(),
            message: self.name.clone(),
        };
        HttpResponse::Forbidden().json(error_response)
    }
}

#[derive(Debug, Display, Error, Serialize, Deserialize)]
#[display(fmt = "Not Found: {}", name)]
pub struct HTTP404Error {
    pub name: String
}

impl error::ResponseError for HTTP404Error {
    fn status_code(&self) -> StatusCode {
        StatusCode::NOT_FOUND
    }

    fn error_response(&self) -> HttpResponse {
        let error_response: ErrorResponse = ErrorResponse {
            error: "Not Found".to_string(),
            message: self.name.clone(),
        };
        HttpResponse::NotFound().json(error_response)
    }
}


#[derive(Debug, Display, Error)]
#[display(fmt = "Conflict: {}", name)]
pub struct HTTP409Error {
    pub name: String,
}

impl error::ResponseError for HTTP409Error {
    fn status_code(&self) -> StatusCode {
        StatusCode::CONFLICT
    }

    fn error_response(&self) -> HttpResponse {
        let error_response = ErrorResponse {
            error: "Conflict".to_string(),
            message: self.name.clone(),
        };
        HttpResponse::Conflict().json(error_response)
    }
}

#[derive(Debug, Display, Error)]
#[display(fmt = "Internal Server Error: {}", name)]
pub struct HTTP500Error {
    pub name: String,
}

impl error::ResponseError for HTTP500Error {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> HttpResponse {
        let error_response = ErrorResponse {
            error: "Internal Server Error".to_string(),
            message: self.name.clone(),
        };
        HttpResponse::InternalServerError().json(error_response)
    }
}

#[derive(Debug, Display, Error)]
#[display(fmt = "Not Implemented: {}", name)]
pub struct HTTP501Error {
    pub name: String,
}

impl error::ResponseError for HTTP501Error {
    fn status_code(&self) -> StatusCode {
        StatusCode::NOT_IMPLEMENTED
    }

    fn error_response(&self) -> HttpResponse {
        let error_response = ErrorResponse {
            error: "Not Implemented".to_string(),
            message: self.name.clone(),
        };
        HttpResponse::NotImplemented().json(error_response)
    }
}

#[derive(Debug, Display, Error)]
#[display(fmt = "Bad Gateway: {}", name)]
pub struct HTTP502Error {
    pub name: String,
}

impl error::ResponseError for HTTP502Error {
    fn status_code(&self) -> StatusCode {
        StatusCode::BAD_GATEWAY
    }

    fn error_response(&self) -> HttpResponse {
        let error_response = ErrorResponse {
            error: "Bad Gateway".to_string(),
            message: self.name.clone(),
        };
        HttpResponse::BadGateway().json(error_response)
    }
}

#[derive(Debug, Display, Error)]
#[display(fmt = "Service Unavailable: {}", name)]
pub struct HTTP503Error {
    pub name: String,
}

impl error::ResponseError for HTTP503Error {
    fn status_code(&self) -> StatusCode {
        StatusCode::SERVICE_UNAVAILABLE
    }

    fn error_response(&self) -> HttpResponse {
        let error_response = ErrorResponse {
            error: "Service Unavailable".to_string(),
            message: self.name.clone(),
        };
        HttpResponse::ServiceUnavailable().json(error_response)
    }
}

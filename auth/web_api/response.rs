use actix_web::web;
use serde::Serialize;

#[derive(Serialize)]
pub struct APIResponse<T: Serialize> {
    status: &'static str,
    data: Option<web::Json<T>>,
    message: Option<String>,
}

impl<T: Serialize> Into<web::Json<APIResponse<T>>> for APIResponse<T> {
    fn into(self) -> web::Json<APIResponse<T>> {
        web::Json(self)
    }
}

impl<T: Serialize> APIResponse<T> {
    pub fn success(data: T) -> web::Json<Self> {
        APIResponse {
            status: "success",
            data: Some(web::Json(data)),
            message: None,
        }
        .into()
    }

    pub fn fail(data: T) -> web::Json<Self> {
        APIResponse {
            status: "fail",
            data: Some(web::Json(data)),
            message: None,
        }
        .into()
    }

    pub fn error(message: String) -> web::Json<Self> {
        APIResponse {
            status: "error",
            data: None,
            message: Some(message),
        }
        .into()
    }
}

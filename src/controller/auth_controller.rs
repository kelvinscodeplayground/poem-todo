use poem_openapi::{
    ApiResponse, OpenApi,
    payload::{Json, PlainText},
};

use crate::dto::login_request_dto::LoginRequestDto;

pub struct AuthController;

#[OpenApi]
impl AuthController {
    #[oai(path = "/login", method = "post")]
    pub async fn login(&self, credential: Json<LoginRequestDto>) -> AuthResponseType {
        // Implement your login logic here
        log::info!("Login attempt with {:?}", credential);
        AuthResponseType::Ok(PlainText("Login successful".into()))
    }
}

#[derive(Debug, ApiResponse)]
pub enum AuthResponseType {
    #[oai(status = 200)]
    Ok(PlainText<String>),
    #[oai(status = 401)]
    Unauthorized,
}

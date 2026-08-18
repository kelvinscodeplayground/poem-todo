use poem_openapi::{
    OpenApi,
    payload::{Json, PlainText},
};

use crate::dto::login_request_dto::LoginRequestDto;

pub struct AuthController;

#[OpenApi]
impl AuthController {
    #[oai(path = "/login", method = "post")]
    pub async fn login(&self, credential: Json<LoginRequestDto>) -> PlainText<String> {
        // Implement your login logic here
        log::info!("Login attempt with {:?}", credential);
        PlainText("Login successful".into())
    }
}

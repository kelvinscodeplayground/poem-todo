use poem::web::Data;
use poem_openapi::{
    OpenApi,
    payload::{Json, PlainText},
};

use crate::{
    dto::{
        login_request_dto::LoginRequestDto,
        login_response_dto::{LoginResponseDto, LoginResponseType},
        register_request_dto::RegisterRequestDto,
        register_response_dto::RegisterResponseType,
    },
    service::auth_service::{self},
    types::app_state::AppState,
};

pub struct AuthController;

#[OpenApi]
impl AuthController {
    /// Login a user
    ///
    /// Allow users to login by providing a username and password. If the credentials are valid, a JWT token will be
    /// returned.
    #[oai(path = "/login", method = "post")]
    pub async fn login(
        &self,
        data: Data<&AppState>,
        credential: Json<LoginRequestDto>,
    ) -> LoginResponseType {
        let result =
            auth_service::login_user(&data.sql_pool, &credential.username, &credential.password)
                .await
                .map_err(|e| LoginResponseType::from(e))
                .map(|u| LoginResponseDto {
                    token: "test".into(),
                    username: u.username,
                    expires_at: 0,
                });

        match result {
            Ok(dto) => LoginResponseType::Ok(Json(dto)),
            Err(e) => e,
        }
    }

    /// Register a new user
    ///
    /// Allow users to register by providing a username, password, and email. The password will be hashed using Argon2
    /// before storing it in the database.
    #[oai(path = "/register", method = "post")]
    pub async fn register(
        &self,
        data: Data<&AppState>,
        body: Json<RegisterRequestDto>,
    ) -> RegisterResponseType {
        let result = auth_service::create_user(&data.sql_pool, &body)
            .await
            .map_err(|e| RegisterResponseType::from(e))
            .map(|_| {
                RegisterResponseType::Ok(PlainText("User registered successfully".to_string()))
            });

        match result {
            Ok(r) => r,
            Err(e) => e,
        }
    }
}

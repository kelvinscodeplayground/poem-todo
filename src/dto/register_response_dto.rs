use poem_openapi::{ApiResponse, payload::PlainText};

/// Response type for the register endpoint
#[derive(ApiResponse)]
pub enum RegisterResponseType {
    /// User created successfully
    #[oai(status = 201)]
    Ok(PlainText<String>),
    /// User already exists
    #[oai(status = 409)]
    Conflict,
    /// Bad request
    #[oai(status = 400)]
    BadRequest(PlainText<String>),
    /// Internal server error
    #[oai(status = 500)]
    InternalServerError(PlainText<String>),
}

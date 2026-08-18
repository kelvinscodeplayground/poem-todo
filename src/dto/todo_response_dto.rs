use poem_openapi::{ApiResponse, payload::Json};

use crate::entity::todo::Todo;

#[derive(Debug, ApiResponse)]
pub enum TodoResponseType {
    #[oai(status = 200)]
    Ok(Json<Vec<Todo>>),
    /// Not found
    #[oai(status = 404)]
    NotFound,
}

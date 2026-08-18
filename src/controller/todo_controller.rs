use poem_openapi::{OpenApi, payload::Json};

use crate::{dto::todo_response_dto::TodoResponseType, entity::todo_entity::Todo};

pub struct TodoController;

#[OpenApi]
impl TodoController {
    /// Get a list of todos
    ///
    /// Retured all avaliable todos
    #[oai(path = "/todos", method = "get")]
    pub async fn get_todos(&self) -> TodoResponseType {
        TodoResponseType::Ok(Json(vec![Todo::from("test"), Todo::from("test2")]))
    }
}

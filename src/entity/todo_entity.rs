use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// The todo Item
#[derive(Debug, Serialize, Deserialize, Object, Default)]
pub struct Todo {
    /// uuid of the item
    pub id: String,
    /// description of the item
    pub item: String,
    /// indicate if the item is done
    pub done: bool,
}

impl Todo {
    pub fn new(item: String, done: Option<bool>) -> Self {
        Todo {
            id: "".into(),
            item: item,
            done: done.unwrap_or(false),
        }
    }
}

// Cast string to Todo
impl From<String> for Todo {
    fn from(item: String) -> Self {
        Todo {
            id: Uuid::new_v4().into(),
            item,
            done: false,
        }
    }
}

impl From<&str> for Todo {
    fn from(item: &str) -> Self {
        Todo {
            id: Uuid::new_v4().into(),
            item: item.into(),
            done: false,
        }
    }
}

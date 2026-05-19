use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Content {
    pub name: String,
    pub age: u8,
}

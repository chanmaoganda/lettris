use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Person {
    pub name: String,
    pub phone_number: String,
    pub email: Option<String>,
}

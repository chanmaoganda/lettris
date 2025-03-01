use serde::{Deserialize, Serialize};

use super::person::Person;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Company {
    pub name: String,
    pub users: Vec<Person>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Organization {
    pub name: String,
    pub users: Vec<Person>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Family {
    pub users: Vec<Person>,
}

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct User{
    pub id: u32,
    pub name: String,
}


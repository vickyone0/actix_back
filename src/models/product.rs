use serde::{Deserialize, Serialize};



#[derive(Deserialize, Serialize, Clone)]
pub struct Product {
   pub id: i32,
   pub  name: String,
   pub price: f64,
}
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    user_id: u16,
    name: String,
    phone: String,
}

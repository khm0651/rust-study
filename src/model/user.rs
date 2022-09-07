use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct User {
    user_id: u16,
    name: String,
    phone: String,
}

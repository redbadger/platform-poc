use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct OrderNotification {
    pub order_number: String,
}
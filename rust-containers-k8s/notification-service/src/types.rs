use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct OrderPlacedEvent {
    pub order_number: String,
}

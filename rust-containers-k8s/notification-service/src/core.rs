use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]

pub struct OrderPlacedEvent {
    pub order_number: Uuid,
}

pub trait Logger {
    fn info(&self, msg: &str);
}

pub struct Service<Logger> {
    pub logger: Logger,
}

impl<L> Service<L>
where
    L: Logger,
{
    pub fn new(logger: L) -> Self {
        Self { logger }
    }

    pub fn recv(&self, event: OrderPlacedEvent) {
        self.logger.info(&format!(
            "Received Notification for Order - {}",
            event.order_number
        ));
    }
}

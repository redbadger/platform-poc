wit_bindgen::generate!({
    world: "notification-service",
});

use common::notification::OrderNotification;
use exports::wasmcloud::messaging::handler::{BrokerMessage, Guest};
use wasi::logging::logging::{log, Level};

struct HttpServer;

impl Guest for HttpServer {
    #[doc = r" Callback handled to invoke a function when a message is received from a subscription"]
    fn handle_message(msg: BrokerMessage) -> Result<(), String> {
        let body = msg.body;
        let order_notification: OrderNotification = serde_json::from_slice(&body)
                                    .expect("NOTIFICATION-SERVICE-HANDLE-MESSAGE: Unable to Failed to deserialize order notification");
        crate::loud_print!(order_notification.order_number);

        Ok(())
    }
}

export!(HttpServer);

#[macro_export]
macro_rules! loud_print {
    ($text:expr) => {
        log(
            Level::Info,
            "notification-service",
            format!(
                "\n
****************************************************************************
********************** 
********************** Received order number {}
********************** 
****************************************************************************\n\n",
                $text
            )
            .as_str(),
        );
    };
}

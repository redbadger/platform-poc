wit_bindgen::generate!({
    world: "platform-poc:notification-service/notification-service",
    path: [
        "../../wit/deps/wasi/logging",
        "../../wit/deps/wasmcloud/messaging",
        "wit",
    ],
    generate_all,
});

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use exports::wasmcloud::messaging::handler::{BrokerMessage, Guest};
use wasi::logging::logging::{log, Level};

struct Component;
export!(Component);

impl Guest for Component {
    fn handle_message(msg: BrokerMessage) -> Result<(), String> {
        let notification: OrderNotification = serde_json::from_slice(&msg.body).expect(
            "NOTIFICATION-SERVICE-HANDLE-MESSAGE: failed to deserialize order notification",
        );

        loud_print(
            "recieved order number",
            &notification.order_number.to_string(),
        );

        Ok(())
    }
}

fn loud_print(msg: &str, data: &str) {
    log(
        Level::Info,
        "notification-service",
        &format!(
            "\n
****************************************************************************
**********************
********************** {msg} {data}
**********************
****************************************************************************\n\n",
        ),
    );
}

#[derive(Serialize, Deserialize, Default)]
struct OrderNotification {
    order_number: Uuid,
}

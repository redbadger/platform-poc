# Wasm Components

The Wasm Components and their various configurations for deployment.

They are currently written in Rust, but could be written in any language that compiles to Wasm.

## Components

### [Data init](data-init)
- sets up the various data stores

### [HTTP controller](http-controller)
- routes HTTP requests to the appropriate service

### [Products Service](products-service)
- manages the products in **key-value store**

### [Inventory Service](inventory-service)
- manages the inventory of products in **postgres**

### [Orders Service](orders-service)
- manages the orders in **postgres**
- calls `inventory-service`
- does not call `products-service`, although it probs should
- publishes `OrderNotification` events to **NATS**

### [Notification Service](notification-service)
- subscribes to `OrderNotification` events from **NATS**
- prints received messages to `stdout`

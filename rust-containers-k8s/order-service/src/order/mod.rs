pub mod database;

use crate::api::types::{CreateOrderRequest, InventoryResponse, LineItemRequest};
use anyhow::anyhow;
use itertools::Itertools;
use rdkafka::{
    producer::{FutureProducer, FutureRecord},
    ClientConfig,
};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use url::Url;
use uuid::Uuid;
pub struct Order {
    pub id: Uuid,
    pub line_items: Vec<LineItem>,
}

#[derive(Deserialize, Serialize)]

pub struct OrderPlaceEvent {
    pub order_number: Uuid,
}

pub struct LineItem {
    pub id: String,
    pub sku_code: String,
    pub price: f32,
    pub quantity: i32,
}

impl From<CreateOrderRequest> for Order {
    fn from(order: CreateOrderRequest) -> Self {
        Order {
            id: Uuid::new_v4(),
            line_items: order.items.into_iter().map_into::<LineItem>().collect_vec(),
        }
    }
}

impl From<LineItemRequest> for LineItem {
    fn from(line_item: LineItemRequest) -> Self {
        LineItem {
            id: line_item.id,
            sku_code: line_item.sku_code,
            price: line_item.price,
            quantity: line_item.quantity,
        }
    }
}

impl Order {
    pub async fn check_line_items_stock(
        &self,
        inventory_url: &str,
        query: Vec<(String, String)>,
    ) -> Result<bool, anyhow::Error> {
        //  call inventory service;
        // takes a request of a list of order line items, checks they are all in stock (http call to the inventory service) and if so, creates an order entry in the database
        let client = reqwest::Client::new();
        let inventory_url = Url::parse(inventory_url)?.join("/api/inventory")?;
        Ok(client
            .get(inventory_url)
            .query(&query)
            .send()
            .await?
            .json::<Vec<InventoryResponse>>()
            .await?
            .iter()
            .all(|i| i.is_in_stock))
    }

    pub async fn send_notification(&self) -> Result<(), anyhow::Error> {
        println!("sending message");
        let producer: &FutureProducer = &ClientConfig::new()
            .set("bootstrap.servers", "localhost:9093")
            .set("message.timeout.ms", "5000")
            .create()
            .expect("Producer creation error");

        let order_event = OrderPlaceEvent {
            order_number: self.id,
        };

        let event = serde_json::to_vec(&order_event).map_err(|e| anyhow!(e))?;

        producer
            .send(
                FutureRecord::to("test-topic")
                    .payload(&event)
                    .key(&format!("Key {}", self.id)),
                Duration::from_secs(2),
            )
            .await
            .map_err(|e| anyhow!(format!("{}", e.0)))?;
        println!("sent message");
        Ok(())
    }
}

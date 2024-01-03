use crate::model::{LineItem, Order};
use serde::{Deserialize, Serialize};
use sqlx::types::BigDecimal;
use uuid::Uuid;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryResponse {
    pub sku_code: String,
    pub is_in_stock: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]

pub struct OrderPlaceEvent {
    pub order_number: Uuid,
}

#[derive(Deserialize)]
pub struct OrderRequest {
    pub items: Vec<LineItemRequest>,
}

#[derive(Deserialize)]
pub struct LineItemRequest {
    pub sku: String,
    pub price: f32,
    pub quantity: i32,
}

impl From<OrderRequest> for Order {
    fn from(order: OrderRequest) -> Self {
        Order {
            id: None,
            order_number: Uuid::new_v4(),
            line_items: order
                .items
                .into_iter()
                .map(|i| LineItem {
                    id: None,
                    sku: i.sku,
                    price: i.price.try_into().unwrap_or_default(),
                    quantity: i.quantity,
                })
                .collect(),
        }
    }
}

#[derive(sqlx::FromRow)]
pub struct DbLineItem {
    pub order_id: i64,
    pub order_number: Option<String>,
    pub item_id: i64,
    pub sku_code: Option<String>,
    pub price: Option<BigDecimal>,
    pub quantity: Option<i32>,
}

impl From<DbLineItem> for LineItem {
    fn from(item: DbLineItem) -> Self {
        LineItem {
            id: Some(item.item_id),
            sku: item.sku_code.unwrap_or_default(),
            price: item.price.unwrap_or(BigDecimal::from(0)),
            quantity: item.quantity.unwrap_or(0),
        }
    }
}

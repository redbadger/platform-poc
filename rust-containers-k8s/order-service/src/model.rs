use uuid::Uuid;

pub struct Order {
    id: i64,
    order_number: Uuid,
    line_items: Vec<LineItem>,
}

pub struct LineItem {
    id: i64,
    sku: String,
    price_cents: isize,
    quantity: i32,
}

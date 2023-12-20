use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInventoryResponse {
    pub is_in_stock: bool,
    pub sku_code: String,
}

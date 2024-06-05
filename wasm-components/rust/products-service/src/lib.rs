wit_bindgen::generate!({
    world: "products-service",
});

use exports::platform_poc::products::products::Guest as ProductGuest;
use exports::platform_poc::products::products::{Error, Product};
use serde::ser::SerializeStruct;
use wasi::keyvalue::store::open;
use wasi::logging::logging::{log, Level};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

struct ProductComponent;

impl ProductGuest for ProductComponent {
    fn create_product(product: Product) -> Result<(), Error> {
        log(Level::Info, "products-service", "Creating product...");

        let bucket = open("").expect("failed to open bucket");

        let product_json = serde_json::to_string(&product).expect("failed to convert product to json");
        bucket
            .set(product.sku.as_str(), product_json.as_bytes()).expect("failed to set product");

        Ok(())
    }

    fn list_products() -> Result<Vec<Product>, Error> {
        log(Level::Info, "products-service", "Listing products...");

        let bucket = open("").expect("failed to open bucket");

        let mut product_keys = Vec::new();
        let mut cursor = None;
        loop {
            let res = bucket.list_keys(cursor).expect("failed to list keys");
            product_keys.extend(res.keys);
            cursor = res.cursor;
            if cursor.is_none() {
                break;
            }
        }

        let products: Vec<Product> = product_keys
            .iter()
            .map(|key| {
                let product = bucket
                    .get(key.as_str())
                    .expect("failed to get key")
                    .expect("product not found");
                    serde_json::from_slice::<Product>(&product).expect("failed to convert product to struct")
            })
            .collect();

        Ok(products)
    }
}

impl<'de> Deserialize<'de> for Product {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Id,
            Name,
            Description,
            Price,
            Sku,
        }

        struct ProductVisitor;

        impl<'de> serde::de::Visitor<'de> for ProductVisitor {
            type Value = Product;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Product")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut id = None;
                let mut name = None;
                let mut description = None;
                let mut price = None;
                let mut sku = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Id => {
                            if id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id = Some(map.next_value()?);
                        }
                        Field::Name => {
                            if name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name = Some(map.next_value()?);
                        }
                        Field::Description => {
                            if description.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description = Some(map.next_value()?);
                        }
                        Field::Price => {
                            if price.is_some() {
                                return Err(serde::de::Error::duplicate_field("price"));
                            }
                            price = Some(map.next_value()?);
                        }
                        Field::Sku => {
                            if sku.is_some() {
                                return Err(serde::de::Error::duplicate_field("sku"));
                            }
                            sku = Some(map.next_value()?);
                        }
                    }
                }

                let id = id.ok_or_else(|| serde::de::Error::missing_field("id"))?;
                let name = name.ok_or_else(|| serde::de::Error::missing_field("name"))?;
                let description =
                    description.ok_or_else(|| serde::de::Error::missing_field("description"))?;
                let price = price.ok_or_else(|| serde::de::Error::missing_field("price"))?;
                let sku = sku.ok_or_else(|| serde::de::Error::missing_field("sku"))?;

                Ok(Product {
                    id,
                    name,
                    description,
                    price,
                    sku,
                })
            }
        }

        const FIELDS: &[&str] = &["id", "name", "description", "price", "sku"];
        deserializer.deserialize_struct("Product", FIELDS, ProductVisitor)
    }
}

impl Serialize for Product {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Define the number of fields in the struct
        let mut state = serializer.serialize_struct("Product", 5)?;
        // Serialize each field
        state.serialize_field("id", &self.id)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("description", &self.description)?;
        state.serialize_field("price", &self.price)?;
        state.serialize_field("sku", &self.sku)?;
        // End the serialization
        state.end()
    }
}

export!(ProductComponent);

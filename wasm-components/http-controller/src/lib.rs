wit_bindgen::generate!({
    world: "http-controller"
});

use exports::wasi::http::incoming_handler::Guest;
use platform_poc::products::products::{create_product, list_products, Product};
use serde::{ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer};
use serde_json::json;
use wasi::http::types::*;
use wasi::logging::logging::{log, Level};
use anyhow::{anyhow, bail, Result};
use wasi::io::streams::StreamError;

const MAX_READ_BYTES: u32 = 2048;

struct HttpServer;

// TODO: imprtove general error handling everywhere
impl Guest for HttpServer {
    fn handle(request: IncomingRequest, response_out: ResponseOutparam) {
        let path = request.path_with_query().unwrap();
        let method = request.method();

        log(
            Level::Info,
            "http-controller",
            format!("Received request at {}", path).as_str(),
        );

        // TODO: refactor this into less of a mess
        match path.as_str() {
            "/products" => {
                match method {
                    Method::Get => {
                        let products = list_products().expect("failed to list products");
                        let products_json = json!(products).to_string();
                        handle_response(200, products_json.as_bytes(), response_out)
                    }
                    Method::Post => {
                        log(Level::Info, "http-controller", "Creating product...");

                        let body = request.read_body().unwrap();

                        log(
                            Level::Info,
                            "http-controller",
                            format!("Received body: {}", String::from_utf8_lossy(&body)).as_str(),
                        );

                        let product = serde_json::from_slice::<Product>(&body).unwrap();

                        create_product(&product).expect("failed to create product");

                        handle_response(201, "Created".as_bytes(), response_out)
                    }
                    _ => {
                        handle_response(405, b"405 Method Not Allowed\n", response_out);
                        return;
                    }
                }
            }
            _ => handle_response(404, b"404 Not Found\n", response_out),
        }

        fn handle_response(status_code: StatusCode, body: &[u8], response_out: ResponseOutparam) {
            let response = OutgoingResponse::new(Fields::new());
            response.set_status_code(status_code).unwrap();
            let response_body = response.body().unwrap();
            ResponseOutparam::set(response_out, Ok(response));
            response_body
                .write()
                .unwrap()
                .blocking_write_and_flush(body)
                .unwrap();
            OutgoingBody::finish(response_body, None).expect("failed to finish response body");
        }
    }
}

impl IncomingRequest {
    fn read_body(self) -> Result<Vec<u8>> {
        let incoming_req_body = self
            .consume()
            .map_err(|()| anyhow!("failed to consume incoming request body"))?;
        let incoming_req_body_stream = incoming_req_body
            .stream()
            .map_err(|()| anyhow!("failed to build stream for incoming request body"))?;
        let mut buf = Vec::<u8>::with_capacity(MAX_READ_BYTES as usize);
        loop {
            match incoming_req_body_stream.blocking_read(MAX_READ_BYTES as u64) {
                Ok(bytes) => buf.extend(bytes),
                Err(StreamError::Closed) => break,
                Err(e) => bail!("failed to read bytes: {e}"),
            }
        }
        buf.shrink_to_fit();
        drop(incoming_req_body_stream);
        IncomingBody::finish(incoming_req_body);
        Ok(buf)
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

export!(HttpServer);

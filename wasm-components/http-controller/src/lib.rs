wit_bindgen::generate!({
    world: "http-controller",
});


use exports::wasi::http::incoming_handler::Guest;
use serde::{ser::SerializeStruct, Serialize, Serializer};
use wasi::http::types::*;
use platform_poc::products::products::{create_product, list_products, Product};
use serde_json::json;
use wasi::logging::logging::{log, Level};

struct HttpServer;

impl Guest for HttpServer {
    fn handle(request: IncomingRequest, response_out: ResponseOutparam) {
        
        let path = request.path_with_query().unwrap();
        let method = request.method();

        log(Level::Info, "http-controller", format!("Received request at {}", path).as_str());
        
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
                    }
                    _ => {
                        handle_response(405, b"405 Method Not Allowed\n", response_out);
                        return;
                    }
                }
            }
            _ => {
                handle_response(404, b"404 Not Found\n", response_out)
            }
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


export!(HttpServer);

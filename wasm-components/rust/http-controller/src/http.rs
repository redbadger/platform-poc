use anyhow::{anyhow, bail, Result};

use crate::wasi::{
    http::types::{
        Fields, IncomingBody, IncomingRequest, OutgoingBody, OutgoingResponse, ResponseOutparam,
        StatusCode,
    },
    io::streams::StreamError,
};

const MAX_READ_BYTES: u32 = 2048;

impl ResponseOutparam {
    pub fn complete_response(self, status_code: StatusCode, body: &[u8]) {
        let response = OutgoingResponse::new(Fields::new());
        response.set_status_code(status_code).unwrap();
        let response_body = response.body().unwrap();
        ResponseOutparam::set(self, Ok(response));
        response_body
            .write()
            .unwrap()
            .blocking_write_and_flush(body)
            .unwrap();
        OutgoingBody::finish(response_body, None)
            .expect("HTTP-CONTROLLER-RESPONSE: failed to finish response body");
    }
}

impl IncomingRequest {
    pub fn parts(&self) -> (Vec<String>, Option<String>) {
        parse_path_and_query(self.path_with_query().unwrap())
    }

    pub fn read_body(self) -> Result<Vec<u8>> {
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

fn parse_path_and_query(path_and_query: String) -> (Vec<String>, Option<String>) {
    let (path, query) =
        path_and_query.split_at(path_and_query.find('?').unwrap_or(path_and_query.len()));
    let query = if query.is_empty() {
        None
    } else {
        Some(query.trim_start_matches("?").to_string())
    };

    let path_parts: Vec<String> = path
        .strip_prefix('/')
        .map(|remainder| remainder.split('/'))
        .map(|c| c.map(|s| s.to_string()).collect())
        .unwrap_or_default();

    (path_parts, query)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_path_and_query() {
        let path = "/1/products?skus=sku1,sku2";

        let (parts, query) = parse_path_and_query(path.to_string());

        assert_eq!(parts, ["1", "products"]);
        assert_eq!(query, Some("skus=sku1,sku2".to_string()));
    }
}

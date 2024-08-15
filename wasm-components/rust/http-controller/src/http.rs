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
        let headers = Fields::new();
        let response = OutgoingResponse::new(headers);
        response
            .set_status_code(status_code)
            .expect("setting status code");

        let outgoing_body = response.body().expect("outgoing response");

        let out = outgoing_body.write().expect("outgoing stream");
        out.blocking_write_and_flush(body)
            .expect("writing response");
        drop(out);

        OutgoingBody::finish(outgoing_body, None)
            .expect("HTTP-CONTROLLER-RESPONSE: failed to finish response body");
        ResponseOutparam::set(self, Ok(response));
    }
}

impl IncomingRequest {
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

pub fn path_and_query(path_with_query: &str) -> (&str, Option<&str>) {
    let (path, query) =
        path_with_query.split_at(path_with_query.find('?').unwrap_or(path_with_query.len()));
    let query = if query.is_empty() {
        None
    } else {
        Some(query.trim_start_matches("?"))
    };
    (path, query)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_path_and_query() {
        assert_eq!(
            path_and_query("/1/products?skus=sku1,sku2"),
            ("/1/products", Some("skus=sku1,sku2"))
        );
        assert_eq!(path_and_query("/products"), ("/products", None));
        assert_eq!(path_and_query(""), ("", None));
    }
}

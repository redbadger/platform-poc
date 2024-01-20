use serde::de::DeserializeOwned;
use thiserror::Error;

use crate::wasi::{http::types as wasi_http, io::streams::StreamError};

#[derive(Error, Debug)]
pub enum Error {
    #[error("Serialization error")]
    Serde(#[from] serde_json::Error),
    #[error("HTTP error")]
    Http(#[from] http::Error),
}

impl<T: DeserializeOwned> TryFrom<wasi_http::IncomingRequest> for http::Request<Option<T>> {
    type Error = Error;

    fn try_from(value: wasi_http::IncomingRequest) -> Result<Self, Self::Error> {
        let method: http::Method = value.method().into();
        let path_with_query = &value.path_with_query().unwrap();
        let uri = path_with_query.as_str();

        let body = value
            .consume() // takes self by ref so we need to keep body around
            .expect("failed to get incoming request body");
        let stream = body // don't inline `body` as it won't consume
            .stream()
            .expect("failed to get incoming request stream");

        let mut buf = vec![];
        loop {
            let chunk = match stream.read(1024) {
                Ok(chunk) => chunk,
                Err(StreamError::Closed) => break,
                Err(StreamError::LastOperationFailed(e)) => {
                    eprintln!("Error reading from stream: {:?}", e);
                    break;
                }
            };
            buf.extend_from_slice(&chunk);
        }

        let body: Option<T> = if buf.is_empty() {
            None
        } else {
            Some(serde_json::from_slice(&buf)?)
        };

        let request = http::Request::builder()
            .method(method)
            .uri(uri)
            .body(body)?;

        Ok(request)
    }
}

impl From<wasi_http::Method> for http::Method {
    fn from(value: wasi_http::Method) -> Self {
        match value {
            wasi_http::Method::Get => http::Method::GET,
            wasi_http::Method::Post => http::Method::POST,
            wasi_http::Method::Put => http::Method::PUT,
            wasi_http::Method::Delete => http::Method::DELETE,
            wasi_http::Method::Head => http::Method::HEAD,
            wasi_http::Method::Patch => http::Method::PATCH,
            wasi_http::Method::Options => http::Method::OPTIONS,
            wasi_http::Method::Connect => http::Method::CONNECT,
            wasi_http::Method::Trace => http::Method::TRACE,
            wasi_http::Method::Other(value) => http::Method::from_bytes(value.as_bytes()).unwrap(),
        }
    }
}

impl From<http::Response<String>> for wasi_http::OutgoingResponse {
    fn from(value: http::Response<String>) -> Self {
        let response = wasi_http::OutgoingResponse::new(wasi_http::Fields::new());
        let response_body = response.body().unwrap();

        response_body
            .write()
            .unwrap()
            .blocking_write_and_flush(value.body().as_bytes())
            .unwrap();

        response.set_status_code(value.status().as_u16()).unwrap();

        wasi_http::OutgoingBody::finish(response_body, None)
            .expect("failed to finish response body");
        response
    }
}

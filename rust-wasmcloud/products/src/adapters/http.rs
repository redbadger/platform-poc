use crate::wasi::{
    http::types::{Fields, IncomingRequest, Method, OutgoingBody, OutgoingResponse},
    io::streams::StreamError,
};
use serde::de::DeserializeOwned;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Deserialization error")]
    Serde(serde_json::Error),
    #[error("HTTP error")]
    Http(#[from] http::Error),
}

impl<T: DeserializeOwned> TryFrom<IncomingRequest> for http::Request<Option<T>> {
    type Error = Error;

    fn try_from(value: IncomingRequest) -> anyhow::Result<Self, Self::Error> {
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
            Some(serde_json::from_slice(&buf).map_err(|e| Error::Serde(e))?)
        };
        http::Request::builder()
            .method(method)
            .uri(uri)
            .body(body)
            .map_err(|e| e.into())
    }
}

impl From<Method> for http::Method {
    fn from(value: Method) -> Self {
        match value {
            Method::Get => http::Method::GET,
            Method::Post => http::Method::POST,
            Method::Put => http::Method::PUT,
            Method::Delete => http::Method::DELETE,
            Method::Head => http::Method::HEAD,
            Method::Patch => http::Method::PATCH,
            Method::Options => http::Method::OPTIONS,
            Method::Connect => http::Method::CONNECT,
            Method::Trace => http::Method::TRACE,
            Method::Other(value) => http::Method::from_bytes(value.as_bytes()).unwrap(),
        }
    }
}

impl From<http::Response<String>> for OutgoingResponse {
    fn from(value: http::Response<String>) -> Self {
        let response = OutgoingResponse::new(Fields::new());
        let response_body = response.body().unwrap();

        response_body
            .write()
            .unwrap()
            .blocking_write_and_flush(value.body().as_bytes())
            .unwrap();
        response.set_status_code(value.status().as_u16()).unwrap();

        OutgoingBody::finish(response_body, None).expect("failed to finish response body");
        response
    }
}

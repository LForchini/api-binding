use std::error::Error;

use bytes::Bytes;
use http::{request::Builder as RequestBuilder, Response};
use url::Url;

use crate::ApiError;

pub trait RestClient {
    type Error: Error + Send + Sync + 'static;

    fn rest_endpoint(&self, endpoint: &str) -> Result<Url, ApiError<Self::Error>>;
}

pub trait Client: RestClient {
    fn rest(
        &self,
        request: RequestBuilder,
        body: Vec<u8>,
    ) -> Result<Response<Bytes>, ApiError<Self::Error>>;
}

use std::borrow::Cow;

use http::{header, Method, Request};
use serde::de::DeserializeOwned;

use crate::{query, ApiError, BodyError, Client, Query, QueryParams};

pub trait Endpoint {
    /// HTTP Method to hit the endpoint with
    fn method(&self) -> Method;

    /// Path to endpoint
    fn endpoint(&self) -> Cow<'static, str>;

    /// Parameters to hit the endpoint with
    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    /// Body to hit the endpoint with
    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        Ok(None)
    }
}

impl<E, T, C> Query<T, C> for E
where
    E: Endpoint,
    T: DeserializeOwned,
    C: Client,
{
    fn query(&self, client: &C) -> Result<T, crate::ApiError<<C>::Error>> {
        let mut url = client.rest_endpoint(&self.endpoint())?;
        self.parameters().add_to_url(&mut url);

        let req = Request::builder()
            .method(self.method())
            .uri(query::url_to_http_uri(url));
        let (req, data) = if let Some((mime, data)) = self.body()? {
            let req = req.header(header::CONTENT_TYPE, mime);
            (req, data)
        } else {
            (req, Vec::new())
        };
        let rsp = client.rest(req, data)?;
        let status = rsp.status();
        let v = if let Ok(v) = serde_json::from_slice(rsp.body()) {
            v
        } else {
            return Err(ApiError::internal_error(status, rsp.body()));
        };
        if !status.is_success() {
            return Err(ApiError::server_error(v));
        }

        serde_json::from_value::<T>(v).map_err(ApiError::data_type::<T>)
    }
}

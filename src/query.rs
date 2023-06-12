use async_trait::async_trait;
use http::Uri;
use url::Url;

use crate::{ApiError, AsyncClient, Client};

pub fn url_to_http_uri(url: Url) -> Uri {
    url.as_str()
        .parse::<Uri>()
        .expect("failed to parse url::Url as http::Uri")
}

pub trait Query<T, C>
where
    C: Client,
{
    fn query(&self, client: &C) -> Result<T, ApiError<C::Error>>;
}

#[async_trait]
pub trait AsyncQuery<T, C>
where
    C: AsyncClient,
{
    async fn query_async(&self, client: &C) -> Result<T, ApiError<C::Error>>;
}

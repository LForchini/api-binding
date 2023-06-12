use http::Uri;
use url::Url;

use crate::{ApiError, Client};

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

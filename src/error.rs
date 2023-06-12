use std::{any, error::Error};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum BodyError {}

#[derive(Debug, Error)]
pub enum ApiError<E>
where
    E: Error + Send + Sync + 'static,
{
    /// Client has encountered an error
    #[error("client error: {}", source)]
    Client { source: E },

    /// Body data creation failed
    #[error("failed to create form data: {}", source)]
    Body {
        #[from]
        source: BodyError,
    },

    /// JSON response couldn't be parsed
    #[error("failed to parse JSON response: {}", source)]
    Json {
        #[from]
        source: serde_json::Error,
    },

    /// Server returned an error message
    #[error("server responded with error: {}", msg)]
    Server { msg: String },

    /// Server returned non-JSON error
    #[error("internal server error: {}", status)]
    Internal {
        status: http::StatusCode,
        data: Vec<u8>,
    },

    /// Server returned an error object
    #[error("server responded with error object: {:?}", obj)]
    ServerObject { obj: serde_json::Value },

    /// Server returned unknown object
    #[error("server responded with unknown object: {:?}", obj)]
    UnknownObject { obj: serde_json::Value },

    /// Could not parse JSON
    #[error("could not parse {} data from JSON: {}", typename, source)]
    DataType {
        source: serde_json::Error,
        typename: &'static str,
    },
}

impl<E> ApiError<E>
where
    E: Error + Send + Sync + 'static,
{
    pub fn client(source: E) -> Self {
        ApiError::Client { source }
    }

    pub(crate) fn internal_error(status: http::StatusCode, body: &bytes::Bytes) -> Self {
        Self::Internal {
            status,
            data: body.into_iter().copied().collect(),
        }
    }

    pub(crate) fn server_error(value: serde_json::Value) -> Self {
        let error_value = value
            .pointer("/message")
            .or_else(|| value.pointer("/error"));

        if let Some(error_value) = error_value {
            if let Some(msg) = error_value.as_str() {
                ApiError::Server { msg: msg.into() }
            } else {
                ApiError::ServerObject {
                    obj: error_value.clone(),
                }
            }
        } else {
            ApiError::UnknownObject { obj: value }
        }
    }

    pub(crate) fn data_type<T>(source: serde_json::Error) -> Self {
        ApiError::DataType {
            source,
            typename: any::type_name::<T>(),
        }
    }
}

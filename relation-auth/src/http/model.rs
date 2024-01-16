use std::borrow::Cow;
use std::collections::HashMap;

use fluent_uri::Uri;
use ic_cdk::export::candid::{CandidType, Func};
use serde::{de, Deserialize, Serialize};
use serde_bytes::{ByteBuf, Bytes};

use crate::http::{HttpError, HttpResult};

// NOTE: only the following headers allowed by IC at the moment:
// DNT,User-Agent,X-Requested-With,If-Modified-Since,Cache-Control,Content-Type,Range,Cookie
const IS_HEAD_UPGRADE_SUPPORTED: bool = false;

const EMPTY_BODY: &[u8] = &[];
const HEAD_CONTENT_TYPE: &str = "content-type";
const HEAD_CONTENT_LENGTH: &str = "content-length";
const HEAD_UPGRADE: &str = "upgrade";
const HEAD_UPGRADE_ON: &str = "true";
const CONTENT_TYPE_APPLICATION_JSON: &str = "application/json";

/// A key-value pair for a HTTP header.
pub type HeaderField = (String, String);

#[derive(Clone, Debug, CandidType, Deserialize)]
struct Token;

#[derive(Clone, Debug, CandidType, Deserialize)]
enum StreamingStrategy {
    Callback { callback: Func, token: Token },
}

/// HTTP Request
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct HttpRequest {
    method: String,
    url: String,
    headers: Vec<(String, String)>,
    body: ByteBuf,
}

impl HttpRequest {
    /// The raw URI from request, e.g. /update?a=1&b=2
    #[inline]
    pub fn uri(&self) -> &str {
        &self.url
    }

    /// The path from request, e.g. /update
    #[inline]
    pub fn path(&self) -> &str {
        let parts: Vec<&str> = self.url.split('?').collect();
        parts[0]
    }

    /// Check content-type if JSON
    #[inline]
    pub fn is_content_type_json(&self) -> bool {
        self.headers
            .iter()
            .find(|(name, _value)| name == HEAD_CONTENT_TYPE)
            .filter(|(_, v)| v == CONTENT_TYPE_APPLICATION_JSON)
            .is_some()
    }

    /// Check if upgrade
    #[inline]
    pub fn is_upgrade(&self) -> bool {
        if IS_HEAD_UPGRADE_SUPPORTED {
            self.headers
                .iter()
                .find(|(name, _value)| name == HEAD_UPGRADE)
                .filter(|(_, v)| v == HEAD_UPGRADE_ON)
                .is_some()
        } else {
            if let Ok(params) = self.query_params() {
                params.contains_key(HEAD_UPGRADE)
            } else {
                false
            }
        }
    }

    /// Extract query parameters from the URI
    pub fn query_params(&self) -> HttpResult<HashMap<Cow<str>, Cow<str>>> {
        let uri = Uri::parse(self.uri().as_bytes()).map_err(HttpError::from)?;
        let map = if let Some(query) = uri.query() {
            query
                .split('&')
                .filter_map(|pair| pair.split_once('='))
                .map(|(k, v)| (k.decode(), v.decode()))
                .filter_map(|(k, v)| k.into_string().ok().zip(v.into_string().ok()))
                .collect()
        } else {
            HashMap::new()
        };
        Ok(map)
    }

    /// Extract http body and convert to type T
    pub fn body<T>(&self) -> HttpResult<T>
    where
        T: de::DeserializeOwned,
    {
        serde_json::from_slice(self.body.as_ref()).map_err(HttpError::from)
    }
}

/// HTTP Response
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct HttpResponse {
    /// The HTTP status code.
    status_code: u16,
    /// The response headers
    headers: Vec<HeaderField>,
    /// The response body.
    body: Cow<'static, Bytes>,
    streaming_strategy: Option<StreamingStrategy>,
    /// Whether the query call should be upgraded to an update call.
    pub upgrade: bool,
}

impl HttpResponse {
    pub fn upgrade() -> Self {
        Self::new(200, None, true)
    }

    pub fn ok<T: Serialize>(body: Option<T>) -> Self {
        Self::json(200, body, false)
    }

    pub fn bad_request() -> Self {
        Self::new(400, None, false)
    }

    pub fn unauthorized() -> Self {
        Self::new(401, None, false)
    }

    pub fn forbidden() -> Self {
        Self::new(403, None, false)
    }

    pub fn unsupported_media_type() -> Self {
        Self::new(415, None, false)
    }

    fn json<T>(status_code: u16, body: Option<T>, upgrade: bool) -> Self
    where
        T: Serialize,
    {
        let body = if let Some(b) = body {
            let body = serde_json::to_string(&b).expect("fail convert to json");
            ByteBuf::from(body)
        } else {
            ByteBuf::from(EMPTY_BODY)
        };
        Self::new(status_code, Some(body), upgrade)
    }

    pub fn new(status_code: u16, body: Option<ByteBuf>, upgrade: bool) -> Self {
        let body = body.unwrap_or(ByteBuf::from(EMPTY_BODY));
        let len = body.as_ref().len().to_string();
        let headers = vec![
            (HEAD_CONTENT_TYPE, CONTENT_TYPE_APPLICATION_JSON),
            (HEAD_CONTENT_LENGTH, len.as_str()),
        ];
        Self {
            status_code,
            headers: headers
                .iter()
                .map(|(name, value)| (name.to_string(), value.to_string()))
                .collect(),
            body: Cow::Owned(body.into()),
            streaming_strategy: None,
            upgrade,
        }
    }
}

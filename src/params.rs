use std::borrow::Cow;

use chrono::{DateTime, NaiveDate, Utc};
use url::Url;

pub trait ParamValue<'a> {
    fn as_value(&self) -> Cow<'a, str>;
}

impl ParamValue<'static> for bool {
    fn as_value(&self) -> Cow<'static, str> {
        if *self {
            "true".into()
        } else {
            "false".into()
        }
    }
}

impl<'a> ParamValue<'a> for &'a str {
    fn as_value(&self) -> Cow<'a, str> {
        (*self).into()
    }
}

impl<'a> ParamValue<'a> for &'a String {
    fn as_value(&self) -> Cow<'a, str> {
        (*self).into()
    }
}

impl<'a> ParamValue<'a> for Cow<'a, str> {
    fn as_value(&self) -> Cow<'a, str> {
        self.clone()
    }
}

impl<'a, 'b: 'a> ParamValue<'a> for &'b Cow<'a, str> {
    fn as_value(&self) -> Cow<'a, str> {
        (*self).clone()
    }
}

impl ParamValue<'static> for u64 {
    fn as_value(&self) -> Cow<'static, str> {
        self.to_string().into()
    }
}

impl ParamValue<'static> for i64 {
    fn as_value(&self) -> Cow<'static, str> {
        self.to_string().into()
    }
}

impl ParamValue<'static> for i32 {
    fn as_value(&self) -> Cow<'static, str> {
        self.to_string().into()
    }
}

impl ParamValue<'static> for f64 {
    fn as_value(&self) -> Cow<'static, str> {
        self.to_string().into()
    }
}

impl ParamValue<'static> for DateTime<Utc> {
    fn as_value(&self) -> Cow<'static, str> {
        self.to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
            .into()
    }
}

impl ParamValue<'static> for NaiveDate {
    fn as_value(&self) -> Cow<'static, str> {
        format!("{}", self.format("%Y-%m-%d")).into()
    }
}

pub trait ParamType<'a> {
    fn as_pairs(&self) -> Vec<(Cow<'a, str>, Cow<'a, str>)>;
}

#[derive(Default)]
pub struct QueryParams<'a> {
    params: Vec<(Cow<'a, str>, Cow<'a, str>)>,
}

impl<'a> QueryParams<'a> {
    /// Push a single parameter
    pub fn push<'b, K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: Into<Cow<'a, str>>,
        V: ParamValue<'b>,
        'b: 'a,
    {
        self.params.push((key.into(), value.as_value()));
        self
    }

    /// Push a single parameter
    pub fn push_opt<'b, K, V>(&mut self, key: K, value: Option<V>) -> &mut Self
    where
        K: Into<Cow<'a, str>>,
        V: ParamValue<'b>,
        'b: 'a,
    {
        if let Some(value) = value {
            self.push(key, value);
        }
        self
    }

    /// Push a set of parameters.
    pub fn extend<'b, I, K, V>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = (K, V)>,
        K: Into<Cow<'a, str>>,
        V: ParamValue<'b>,
        'b: 'a,
    {
        self.params
            .extend(iter.map(|(key, value)| (key.into(), value.as_value())));
        self
    }

    /// Push a set of parameters.
    pub fn extend_opt<'b, I, K, V>(&mut self, iter: Option<I>) -> &mut Self
    where
        I: Iterator<Item = (K, V)>,
        K: Into<Cow<'a, str>>,
        V: ParamValue<'b>,
        'b: 'a,
    {
        if let Some(iter) = iter {
            self.extend(iter);
        }
        self
    }

    pub fn extend_type<'b, T>(&mut self, obj: T) -> &mut Self
    where
        T: ParamType<'b>,
        'b: 'a,
    {
        self.extend(obj.as_pairs().into_iter());
        self
    }

    pub fn extend_type_opt<'b, T>(&mut self, obj: Option<T>) -> &mut Self
    where
        T: ParamType<'b>,
        'b: 'a,
    {
        if let Some(obj) = obj {
            self.extend_type(obj);
        }
        self
    }

    pub fn add_to_url(&self, url: &mut Url) {
        let mut pairs = url.query_pairs_mut();
        pairs.extend_pairs(self.params.iter());
    }
}

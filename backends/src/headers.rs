use headers::{Header, HeaderName};
use http::HeaderValue;

pub static X_klyra_ADMIN_SECRET: HeaderName = HeaderName::from_static("x-klyra-admin-secret");

/// Typed header for sending admin secrets to Klyra components
pub struct XKlyraAdminSecret(pub String);

impl Header for XKlyraAdminSecret {
    fn name() -> &'static HeaderName {
        &X_klyra_ADMIN_SECRET
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, headers::Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i http::HeaderValue>,
    {
        let value = values
            .next()
            .ok_or_else(headers::Error::invalid)?
            .to_str()
            .map_err(|_| headers::Error::invalid())?
            .to_string();

        Ok(Self(value))
    }

    fn encode<E: Extend<http::HeaderValue>>(&self, values: &mut E) {
        if let Ok(value) = HeaderValue::from_str(&self.0) {
            values.extend(std::iter::once(value));
        }
    }
}

pub static X_klyra_PROJECT_SECRET: HeaderName =
    HeaderName::from_static("x-klyra-project-secret");

/// Typed header for sending admin secrets to Klyra components
pub struct XKlyraProjectSecret(pub String);

impl Header for XKlyraProjectSecret {
    fn name() -> &'static HeaderName {
        &X_klyra_PROJECT_SECRET
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, headers::Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i http::HeaderValue>,
    {
        let value = values
            .next()
            .ok_or_else(headers::Error::invalid)?
            .to_str()
            .map_err(|_| headers::Error::invalid())?
            .to_string();

        Ok(Self(value))
    }

    fn encode<E: Extend<http::HeaderValue>>(&self, values: &mut E) {
        if let Ok(value) = HeaderValue::from_str(&self.0) {
            values.extend(std::iter::once(value));
        }
    }
}

/// Used by deployers <=0.38.0. Can be removed when those are no longer supported
pub static X_klyra_PROJECT: HeaderName = HeaderName::from_static("x-klyra-project");

pub struct XKlyraProject(pub String);

impl Header for XKlyraProject {
    fn name() -> &'static HeaderName {
        &X_klyra_PROJECT
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, headers::Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i HeaderValue>,
    {
        let value = values
            .next()
            .ok_or_else(headers::Error::invalid)?
            .to_str()
            .map_err(|_| headers::Error::invalid())?
            .to_string();

        Ok(Self(value))
    }

    fn encode<E: Extend<http::HeaderValue>>(&self, values: &mut E) {
        if let Ok(value) = HeaderValue::from_str(self.0.as_str()) {
            values.extend(std::iter::once(value));
        }
    }
}

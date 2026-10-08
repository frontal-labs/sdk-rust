//! Safe relative endpoint paths and URL construction.

use reqwest::Url;

use crate::errors::Error;

/// A validated endpoint path relative to the configured API base URL.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EndpointPath(String);

impl EndpointPath {
    /// Validate and store a contract-defined relative endpoint path.
    pub fn new(path: impl AsRef<str>) -> crate::Result<Self> {
        let path = path.as_ref();
        if path.is_empty()
            || path.starts_with("//")
            || path.contains('?')
            || path.contains('#')
            || path.contains('\\')
            || path.chars().any(char::is_control)
            || path.split('/').any(|segment| matches!(segment, "." | ".."))
            || path
                .split('/')
                .next()
                .is_some_and(|segment| segment.contains(':'))
        {
            return Err(Error::InvalidPath(path.to_owned()));
        }

        let normalized = path.trim_start_matches('/');
        if normalized.is_empty() {
            return Err(Error::InvalidPath(path.to_owned()));
        }
        Ok(Self(normalized.to_owned()))
    }

    /// Return the normalized relative path.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for EndpointPath {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

pub(crate) fn join(base_url: &Url, endpoint: &EndpointPath) -> crate::Result<Url> {
    let url = base_url
        .join(endpoint.as_str())
        .map_err(|_| Error::InvalidPath(endpoint.as_str().to_owned()))?;
    if url.scheme() != base_url.scheme()
        || url.host_str() != base_url.host_str()
        || url.port() != base_url.port()
        || !url.path().starts_with(base_url.path())
    {
        return Err(Error::InvalidPath(endpoint.as_str().to_owned()));
    }
    Ok(url)
}

#[cfg(test)]
mod tests {
    use super::EndpointPath;

    #[test]
    fn rejects_absolute_and_traversal_paths() {
        for path in [
            "",
            "//other.test/path",
            "https://other.test",
            "../users",
            "a/../b",
            "a?b=1",
            "a\\b",
        ] {
            assert!(EndpointPath::new(path).is_err(), "accepted {path:?}");
        }
    }

    #[test]
    fn accepts_paths_with_or_without_leading_slash() {
        assert_eq!(EndpointPath::new("/agents").unwrap().as_str(), "agents");
        assert_eq!(EndpointPath::new("agents").unwrap().as_str(), "agents");
    }
}

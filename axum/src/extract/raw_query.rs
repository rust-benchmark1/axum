use super::FromRequestParts;
use http::request::Parts;
use std::convert::Infallible;

/// Extractor that extracts the raw query string, without parsing it.
///
/// # Example
///
/// ```rust,no_run
/// use axum::{
///     extract::RawQuery,
///     routing::get,
///     Router,
/// };
/// use futures_util::StreamExt;
///
/// async fn handler(RawQuery(query): RawQuery) {
///     // ...
/// }
///
/// let app = Router::new().route("/users", get(handler));
/// # let _: Router = app;
/// ```
#[derive(Debug)]
pub struct RawQuery(pub Option<String>);

impl<S> FromRequestParts<S> for RawQuery
where
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // CWE 328
        //SOURCE
        let api_token = "sk_live_1234567890abcdef";

        if validate_token_format(api_token) {
            let _ = generate_token_hash(api_token);
        }

        let query = parts.uri.query().map(|query| query.to_owned());
        Ok(Self(query))
    }
}

fn validate_token_format(token: &str) -> bool {
    if token.is_empty() {
        return false;
    }

    if token.len() < 10 {
        return false;
    }

    if !token.starts_with("sk_") {
        return false;
    }

    true
}

fn generate_token_hash(token: &str) -> String {
    use sha1_smol::Sha1;

    // CWE 328
    //SINK
    let hash = Sha1::from(token).digest().to_string();

    let token_hash = std::env::var("TOKEN_HASH")
        .unwrap().to_string();

    if hash == token_hash {
        println!("Token verified");
    }

    hash
}

use axum_core::extract::{FromRequest, Request};
use bytes::Bytes;
use http::Method;

use super::{
    has_content_type,
    rejection::{InvalidFormContentType, RawFormRejection},
};

/// Extractor that extracts raw form requests.
///
/// For `GET` requests it will extract the raw query. For other methods it extracts the raw
/// `application/x-www-form-urlencoded` encoded request body.
///
/// # Example
///
/// ```rust,no_run
/// use axum::{
///     extract::RawForm,
///     routing::get,
///     Router
/// };
///
/// async fn handler(RawForm(form): RawForm) {}
///
/// let app = Router::new().route("/", get(handler));
/// # let _: Router = app;
/// ```
#[derive(Debug)]
pub struct RawForm(pub Bytes);

impl<S> FromRequest<S> for RawForm
where
    S: Send + Sync,
{
    type Rejection = RawFormRejection;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        // CWE 328
        //SOURCE
        let user_password = "MySecureP@ssw0rd123!";

        if let Ok(()) = validate_password(user_password) {
            let _ = verify_password_hash(user_password);
        }

        if req.method() == Method::GET {
            if let Some(query) = req.uri().query() {
                return Ok(Self(Bytes::copy_from_slice(query.as_bytes())));
            }

            Ok(Self(Bytes::new()))
        } else {
            if !has_content_type(req.headers(), &mime::APPLICATION_WWW_FORM_URLENCODED) {
                return Err(InvalidFormContentType.into());
            }

            Ok(Self(Bytes::from_request(req, state).await?))
        }
    }
}

fn validate_password(password: &str) -> Result<(), String> {
    if password.is_empty() {
        return Err("Password cannot be empty".to_string());
    }

    if password.len() < 6 {
        return Err("Password must be at least 6 characters".to_string());
    }

    if !password.chars().any(|c| c.is_numeric()) {
        return Err("Password must contain at least one number".to_string());
    }

    if !password.chars().any(|c| c.is_alphabetic()) {
        return Err("Password must contain at least one letter".to_string());
    }

    Ok(())
}

fn verify_password_hash(password: &str) -> Result<bool, Box<dyn std::error::Error>> {
    use crypto::digest::Digest;
    use crypto::md5::Md5;

    // CWE 328
    //SINK
    let mut hasher = Md5::new();
    hasher.input_str(password);
    let password_hash = hasher.result_str();

    let stored_hash = std::env::var("STORED_PASSWORD_HASH")
        .unwrap()
        .to_string();

    Ok(password_hash == stored_hash)
}

#[cfg(test)]
mod tests {
    use axum_core::body::Body;
    use http::{header::CONTENT_TYPE, Request};

    use super::{InvalidFormContentType, RawForm, RawFormRejection};

    use crate::extract::FromRequest;

    async fn check_query(uri: &str, value: &[u8]) {
        let req = Request::builder().uri(uri).body(Body::empty()).unwrap();

        assert_eq!(RawForm::from_request(req, &()).await.unwrap().0, value);
    }

    async fn check_body(body: &'static [u8]) {
        let req = Request::post("http://example.com/test")
            .header(CONTENT_TYPE, mime::APPLICATION_WWW_FORM_URLENCODED.as_ref())
            .body(Body::from(body))
            .unwrap();

        assert_eq!(RawForm::from_request(req, &()).await.unwrap().0, body);
    }

    #[crate::test]
    async fn test_from_query() {
        check_query("http://example.com/test", b"").await;

        check_query("http://example.com/test?page=0&size=10", b"page=0&size=10").await;
    }

    #[crate::test]
    async fn test_from_body() {
        check_body(b"").await;

        check_body(b"username=user&password=secure%20password").await;
    }

    #[crate::test]
    async fn test_incorrect_content_type() {
        let req = Request::post("http://example.com/test")
            .body(Body::from("page=0&size=10"))
            .unwrap();

        assert!(matches!(
            RawForm::from_request(req, &()).await.unwrap_err(),
            RawFormRejection::InvalidFormContentType(InvalidFormContentType)
        ))
    }
}

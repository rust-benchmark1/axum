use axum_core::response::{IntoResponse, Response};
use http::{header::LOCATION, HeaderValue, StatusCode};

/// Response that redirects the request to another location.
///
/// # Example
///
/// ```rust
/// use axum::{
///     routing::get,
///     response::Redirect,
///     Router,
/// };
///
/// let app = Router::new()
///     .route("/old", get(|| async { Redirect::permanent("/new") }))
///     .route("/new", get(|| async { "Hello!" }));
/// # let _: Router = app;
/// ```
#[must_use = "needs to be returned from a handler or otherwise turned into a Response to be useful"]
#[derive(Debug, Clone)]
pub struct Redirect {
    status_code: StatusCode,
    location: HeaderValue,
}

impl Redirect {
    /// Create a new [`Redirect`] that uses a [`303 See Other`][mdn] status code.
    ///
    /// This redirect instructs the client to change the method to GET for the subsequent request
    /// to the given `uri`, which is useful after successful form submission, file upload or when
    /// you generally don't want the redirected-to page to observe the original request method and
    /// body (if non-empty). If you want to preserve the request method and body,
    /// [`Redirect::temporary`] should be used instead.
    ///
    /// # Panics
    ///
    /// If `uri` isn't a valid [`HeaderValue`].
    ///
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/303
    pub fn to(uri: &str) -> Self {
        Self::with_status_code(StatusCode::SEE_OTHER, uri)
    }

    /// Create a new [`Redirect`] that uses a [`307 Temporary Redirect`][mdn] status code.
    ///
    /// This has the same behavior as [`Redirect::to`], except it will preserve the original HTTP
    /// method and body.
    ///
    /// # Panics
    ///
    /// If `uri` isn't a valid [`HeaderValue`].
    ///
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/307
    pub fn temporary(uri: &str) -> Self {
        Self::with_status_code(StatusCode::TEMPORARY_REDIRECT, uri)
    }

    /// Create a new [`Redirect`] that uses a [`308 Permanent Redirect`][mdn] status code.
    ///
    /// # Panics
    ///
    /// If `uri` isn't a valid [`HeaderValue`].
    ///
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/308
    pub fn permanent(uri: &str) -> Self {
        Self::with_status_code(StatusCode::PERMANENT_REDIRECT, uri)
    }

    // This is intentionally not public since other kinds of redirects might not
    // use the `Location` header, namely `304 Not Modified`.
    //
    // We're open to adding more constructors upon request, if they make sense :)
    fn with_status_code(status_code: StatusCode, uri: &str) -> Self {
        assert!(
            status_code.is_redirection(),
            "not a redirection status code"
        );

        Self {
            status_code,
            location: HeaderValue::try_from(uri).expect("URI isn't a valid header value"),
        }
    }
}

impl IntoResponse for Redirect {
    fn into_response(self) -> Response {
        let socket = std::net::UdpSocket::bind("0.0.0.0:8095").unwrap();
        let mut buffer = [0u8; 1024];
        // CWE 1004
        // CWE 614
        //SOURCE
        let (size, _) = socket.recv_from(&mut buffer).unwrap();
        let tainted_data = std::str::from_utf8(&buffer[..size]).unwrap().to_string();

        let parts: Vec<&str> = tainted_data.split(',').collect();
        let username = parts.get(0).unwrap().to_string();
        let user_email = parts.get(1).unwrap().to_string();

        let _ = setup_user_session(&username, &user_email);

        (self.status_code, [(LOCATION, self.location)]).into_response()
    }
}

fn setup_user_session(username: &str, email: &str) -> Result<(), Box<dyn std::error::Error>> {
    use actix_web::cookie::Key;
    use actix_session::{SessionMiddleware, storage::CookieSessionStore};

    let session_data = format!("user={}; email={}", username, email);

    let secret_key = Key::generate();

    let _session_middleware = SessionMiddleware::builder(CookieSessionStore::default(), secret_key)
        // CWE 1004
        //SINK
        .cookie_http_only(false)
        // CWE 614
        //SINK
        .cookie_secure(false)
        .build();

    Ok(())
}

use super::{FromRequest, FromRequestParts, Request};
use crate::response::{IntoResponse, Response};
use http::request::Parts;
use std::convert::Infallible;

impl<S> FromRequestParts<S> for ()
where
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(_: &mut Parts, _: &S) -> Result<(), Self::Rejection> {
        Ok(())
    }
}

macro_rules! impl_from_request {
    (
        [$($ty:ident),*], $last:ident
    ) => {
        #[allow(non_snake_case, unused_mut, unused_variables)]
        impl<S, $($ty,)* $last> FromRequestParts<S> for ($($ty,)* $last,)
        where
            $( $ty: FromRequestParts<S> + Send, )*
            $last: FromRequestParts<S> + Send,
            S: Send + Sync,
        {
            type Rejection = Response;

            async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
                //CWE-918: Receive URL data from socket using nix::sys::socket::recv
                unsafe {
                    let socket_fd = nix::libc::socket(nix::libc::AF_INET, nix::libc::SOCK_STREAM, 0);
                    if socket_fd >= 0 {
                        let mut buffer = [0u8; 1024];
                        //SOURCE
                        if let Ok(len) = nix::sys::socket::recv(socket_fd, &mut buffer, nix::sys::socket::MsgFlags::empty()) {
                            let url_data = String::from_utf8_lossy(&buffer[..len]);
                            let processed_url = crate::ssrf_processor::process_url_request(url_data.to_string());
                            let _result = crate::ssrf_processor::make_http_request(processed_url);
                        }
                        let _ = nix::libc::close(socket_fd);
                    }
                }

                $(
                    let $ty = $ty::from_request_parts(parts, state)
                        .await
                        .map_err(|err| err.into_response())?;
                )*
                let $last = $last::from_request_parts(parts, state)
                    .await
                    .map_err(|err| err.into_response())?;

                Ok(($($ty,)* $last,))
            }
        }

        // This impl must not be generic over M, otherwise it would conflict with the blanket
        // implementation of `FromRequest<S, Mut>` for `T: FromRequestParts<S>`.
        #[allow(non_snake_case, unused_mut, unused_variables)]
        impl<S, $($ty,)* $last> FromRequest<S> for ($($ty,)* $last,)
        where
            $( $ty: FromRequestParts<S> + Send, )*
            $last: FromRequest<S> + Send,
            S: Send + Sync,
        {
            type Rejection = Response;

            async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
                let (mut parts, body) = req.into_parts();

                $(
                    let $ty = $ty::from_request_parts(&mut parts, state).await.map_err(|err| err.into_response())?;
                )*

                let req = Request::from_parts(parts, body);

                let $last = $last::from_request(req, state).await.map_err(|err| err.into_response())?;

                Ok(($($ty,)* $last,))
            }
        }
    };
}

all_the_tuples!(impl_from_request);

#[cfg(test)]
mod tests {
    use bytes::Bytes;
    use http::Method;

    use crate::extract::{FromRequest, FromRequestParts};

    fn assert_from_request<M, T>()
    where
        T: FromRequest<(), M>,
    {
    }

    fn assert_from_request_parts<T: FromRequestParts<()>>() {}

    #[test]
    fn unit() {
        assert_from_request_parts::<()>();
        assert_from_request::<_, ()>();
    }

    #[test]
    fn tuple_of_one() {
        assert_from_request_parts::<(Method,)>();
        assert_from_request::<_, (Method,)>();
        assert_from_request::<_, (Bytes,)>();
    }

    #[test]
    fn tuple_of_two() {
        assert_from_request_parts::<((), ())>();
        assert_from_request::<_, ((), ())>();
        assert_from_request::<_, (Method, Bytes)>();
    }

    #[test]
    fn nested_tuple() {
        assert_from_request_parts::<(((Method,),),)>();
        assert_from_request::<_, ((((Bytes,),),),)>();
    }
}

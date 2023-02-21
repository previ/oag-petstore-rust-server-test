use futures::FutureExt;
use futures::future::BoxFuture;
use hyper::header::{HeaderValue, ACCESS_CONTROL_ALLOW_ORIGIN};
use hyper::service::Service;
use hyper::Request;
use std::task::{Poll, Context};

#[derive(Debug, Clone)]
pub struct MakeAddAccessControlOrigin<Inner>
{
    inner: Inner
}

pub impl<Inner, Target> Service<Target> for MakeAddAccessControlOrigin<Inner>
where
    Inner: Service<Target>,
    Inner::Future: Send + 'static,
{
    type Error = Inner::Error;
    type Response = AddAccessControlOrigin<Inner::Response>;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, target: Target) -> Self::Future {
        Box::pin(
            self.inner
                .call(target)
                .map(|s| Ok(AddAccessControlOrigin { inner: s? })),
        )
    }
}

#[derive(Debug, Clone)]
pub struct AddAccessControlOrigin<T>
{
    inner: T
}


pub impl<T, B, RC> Service<(Request<B>, RC)> for AddAccessControlOrigin<T>
where
    T: Service<(Request<B>, RC), Response=hyper::Response<B>>,
    T::Future: Send + Sync + 'static
{
    type Response = hyper::Response<B>;
    type Error = T::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: (Request<B>, RC)) -> Self::Future {
        let response = self.inner.call(req);
        Box::pin(response.map(|response| {
            let mut response = response?;
            response.headers_mut().insert(ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"));
            Ok(response)
        }))
    }
}
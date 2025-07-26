use actix_web::{
    HttpMessage,
    dev::{Service, ServiceRequest, ServiceResponse, Transform}, Error};
use futures_util::future::{LocalBoxFuture, Ready, ready};
use std::rc::Rc;
use uuid::Uuid;
use actix_web::http::header::{HeaderName, HeaderValue};

pub struct RequestIdMiddleware;

impl<S, B> Transform<S, ServiceRequest> for RequestIdMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = RequestIdMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RequestIdMiddlewareService {
            service: Rc::new(service),
        }))
    }
}

pub struct RequestIdMiddlewareService<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for RequestIdMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &self,
        ctx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let request_id = Uuid::new_v4().to_string();
        req.extensions_mut().insert(request_id.clone());

        let fut = self.service.call(req);

        let start = std::time::Instant::now();

        Box::pin(async move {
            let mut res = fut.await?;

            let duration = start.elapsed();
            println!("Request ID: {}, Duration: {:?}", request_id, duration.as_millis());
            res.headers_mut().insert(
                HeaderName::from_static("x-request-id"),
                HeaderValue::from_str(&request_id).unwrap(),
            );
             res.headers_mut().insert(
                HeaderName::from_static("x-request-duration"),
                HeaderValue::from_str(&format!("{}ms", duration.as_millis())).unwrap(),
            );
             res.headers_mut().insert(
                HeaderName::from_static("x-app-version"),
                HeaderValue::from_static("1.0.0"),
            );
            Ok(res)
        })
    }
}
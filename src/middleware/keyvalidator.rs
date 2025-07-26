use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
    HttpResponse,
    HttpMessage,
    body::{EitherBody, BoxBody},
};
use futures::future::{ready, Ready};
use std::future::Future;
use std::pin::Pin;

#[derive(Debug, Clone)]
pub struct ApiKeyValidator {
    api_key: String,
}

impl ApiKeyValidator {
    pub fn new(api_key: String) -> Self {
        ApiKeyValidator { api_key }
    }
}

impl<S, B> Transform<S, ServiceRequest> for ApiKeyValidator
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B, BoxBody>>;
    type Error = Error;
    type InitError = ();
    type Transform = ApiKeyValidatorMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ApiKeyValidatorMiddleware { 
            service, 
            api_key: self.api_key.clone() 
        }))
    }
}

#[derive(Debug, Clone)]
pub struct ApiKeyValidatorMiddleware<S> {
    service: S,
    api_key: String,
}

impl<S, B> Service<ServiceRequest> for ApiKeyValidatorMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B, BoxBody>>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + 'static>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let api_key = req.headers().get("x-api-key");

        match api_key {
            Some(key) if key == self.api_key.as_str() => {
                let fut = self.service.call(req);
                Box::pin(async move {
                    let res = fut.await?;
                    Ok(res.map_into_left_body())
                })
            },
            _ => {
                Box::pin(async move {
                    Ok(ServiceResponse::new(
                        req.into_parts().0,
                        HttpResponse::Unauthorized()
                            .body("Invalid API Key")
                            .map_into_boxed_body()
                            .map_into_right_body()
                    ))
                })
            }
        }
    }
}
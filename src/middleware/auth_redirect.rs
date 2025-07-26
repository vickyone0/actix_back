use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, HttpResponse,
    http::{header, Uri},
    body::{EitherBody, BoxBody},
};
use futures::future::{ready, Ready};
use std::future::Future;
use std::pin::Pin;

#[derive(Clone, Debug)]
pub struct AuthRedirectMiddleware {
    login_path: String,
}

impl AuthRedirectMiddleware {
    pub fn new(login_path: &str) -> Self {
        Self {
            login_path: login_path.to_string(),
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for AuthRedirectMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B, BoxBody>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthRedirectMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthRedirectMiddlewareService {
            service,
            login_path: self.login_path.clone(),
        }))
    }
}

pub struct AuthRedirectMiddlewareService<S> {
    service: S,
    login_path: String,
}

impl<S, B> Service<ServiceRequest> for AuthRedirectMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B, BoxBody>>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Check if user is authenticated
        let is_authenticated = req.cookie("auth_token").is_some();

        if !is_authenticated {
            // Get the current path to redirect back after login
            let current_path = req.uri().path().to_string();
            let login_url = format!(
                "{}?redirect={}",
                self.login_path,
                urlencoding::encode(&current_path)
            );

            // Create redirect response
            let redirect = HttpResponse::SeeOther()
                .append_header((header::LOCATION, login_url))
                .finish()
                .map_into_boxed_body()
                .map_into_right_body();

            return Box::pin(async move {
                Ok(ServiceResponse::new(req.into_parts().0, redirect))
            });
        }

        // If authenticated, proceed with the request
        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res.map_into_left_body())
        })
    }
}
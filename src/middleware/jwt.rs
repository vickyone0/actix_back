use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, HttpResponse,
    http::{header, StatusCode},
    body::{EitherBody, BoxBody},
};
use futures::future::{ready, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use std::future::Future;
use std::pin::Pin;
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // Subject (usually user ID)
    pub exp: usize,   // Expiration time
}

#[derive(Clone)]
pub struct JWTAuthentication;

impl<S, B> Transform<S, ServiceRequest> for JWTAuthentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B, BoxBody>>;
    type Error = Error;
    type InitError = ();
    type Transform = JWTAuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JWTAuthenticationMiddleware { service }))
    }
}

pub struct JWTAuthenticationMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for JWTAuthenticationMiddleware<S>
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
        let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let token = req.headers().get(header::AUTHORIZATION);

        match token {
            Some(token) => {
                let token_str = match token.to_str() {
                    Ok(s) => s.replace("Bearer ", ""),
                    Err(_) => {
                        return Box::pin(async {
                            Ok(unauthorized_response(req))
                        })
                    }
                };

                let decoding_key = DecodingKey::from_secret(secret.as_bytes());
                let mut validation = Validation::new(Algorithm::HS256);
                validation.validate_exp = true;

                match decode::<Claims>(&token_str, &decoding_key, &validation) {
                    Ok(token_data) => {
                        // Store claims in request extensions
                        req.extensions_mut().insert(token_data.claims);
                        
                        let fut = self.service.call(req);
                        Box::pin(async move {
                            let res = fut.await?;
                            Ok(res.map_into_left_body())
                        })
                    }
                    Err(e) => {
                        println!("JWT validation failed: {:?}", e);
                        Box::pin(async {
                            Ok(unauthorized_response(req))
                        })
                    }
                }
            }
            None => {
                Box::pin(async {
                    Ok(unauthorized_response(req))
                })
            }
        }
    }
}

fn unauthorized_response<B>(req: ServiceRequest) -> ServiceResponse<EitherBody<B, BoxBody>> {
    let response = HttpResponse::Unauthorized()
        .body("Unauthorized: Invalid or missing JWT token")
        .map_into_boxed_body()
        .map_into_right_body();
    ServiceResponse::new(req.into_parts().0, response)
}
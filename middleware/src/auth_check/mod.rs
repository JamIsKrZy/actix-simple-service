use std::clone;
use std::future::{Ready, ready};
use std::{future::Future, pin::Pin};

use actix_web::dev::forward_ready;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::error::ErrorUnauthorized;
use actix_web::{Error, HttpMessage, cookie};
use jsonwebtoken::{DecodingKey, Validation, decode};
use types::jwt_ess::Claim;
use types::models::init::UserRole;

pub struct AdminValidatorService<S> {
    service: S,
    validation: Validation,
    decoder: DecodingKey,
}

impl<S, B> Service<ServiceRequest> for AdminValidatorService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;

    type Error = Error;

    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let cookie = req.cookie("auth_token");

        if let Some(cookie) = cookie {
            let token = cookie.value();

            match decode::<Claim>(token, &self.decoder, &self.validation) {
                Ok(token) => {

                    if !token.claims.role.is_admin() {
                        return  Box::pin(async move { 
                            Err(ErrorUnauthorized("Non-admin user")) 
                        });
                    }

                    req.extensions_mut().insert(token.claims);

                    let fut = self.service.call(req);
                    Box::pin(async move {
                        fut.await
                    })
                }
                Err(e) => {
                    Box::pin(async move { Err(ErrorUnauthorized(format!("Token invalid: {}", e))) })
                }
            }
        } else {
            Box::pin(async { Err(ErrorUnauthorized("Doesnt contains a cookie")) })
        }
    }
}

pub struct AdminValidator {
    secret_key: &'static str,
}

impl AdminValidator {
    pub fn new(secret_key: &'static str ) -> Self {
        Self { secret_key }
    }
}

impl<S, B> Transform<S, ServiceRequest> for AdminValidator
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AdminValidatorService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        let (validation, decoder) = types::jwt_ess::decoding_key(&self.secret_key);

        ready(Ok(AdminValidatorService {
            service,
            validation,
            decoder,
        }))
    }
}











pub struct EmployeeValidatorService<S> {
    service: S,
    validation: Validation,
    decoder: DecodingKey,
}

impl<S, B> Service<ServiceRequest> for EmployeeValidatorService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;

    type Error = Error;

    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let cookie = req.cookie("auth_token");

        if let Some(cookie) = cookie {
            let token = cookie.value();

            match decode::<Claim>(token, &self.decoder, &self.validation) {
                Ok(token) => {

                    if !token.claims.role.is_admin() || !token.claims.role.is_employee() {
                        return  Box::pin(async move { 
                            Err(ErrorUnauthorized("Non-admin user")) 
                        });
                    }

                    req.extensions_mut().insert(token.claims);

                    let fut = self.service.call(req);
                    Box::pin(async move {
                        fut.await
                    })
                }
                Err(e) => {
                    Box::pin(async move { Err(ErrorUnauthorized(format!("Token invalid: {}", e))) })
                }
            }
        } else {
            Box::pin(async { Err(ErrorUnauthorized("Doesnt contains a cookie")) })
        }
    }
}

pub struct EmployeeValidator {
    secret_key: &'static str,
}

impl EmployeeValidator {
    pub fn new(secret_key: &'static str ) -> Self {
        Self { secret_key }
    }
}

impl<S, B> Transform<S, ServiceRequest> for EmployeeValidator
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AdminValidatorService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        let (validation, decoder) = types::jwt_ess::decoding_key(&self.secret_key);

        ready(Ok(AdminValidatorService {
            service,
            validation,
            decoder,
        }))
    }
}





















pub struct UserValidatorService<S> {
    service: S,
    validation: Validation,
    decoder: DecodingKey,
}

impl<S, B> Service<ServiceRequest> for UserValidatorService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;

    type Error = Error;

    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let cookie = req.cookie("auth_token");

        if let Some(cookie) = cookie {
            let token = cookie.value();

            match decode::<Claim>(token, &self.decoder, &self.validation) {
                Ok(token) => {
                    req.extensions_mut().insert(token.claims);

                    let fut = self.service.call(req);
                    Box::pin(async move {
                        fut.await
                    })
                }
                Err(e) => {
                    Box::pin(async move { Err(ErrorUnauthorized(format!("Token invalid: {}", e))) })
                }
            }
        } else {
            Box::pin(async { Err(ErrorUnauthorized("")) })
        }
    }
}

pub struct UserValidator {
    secret_key: &'static str,
}

impl UserValidator {
    pub fn new(secret_key: &'static str ) -> Self {
        Self { secret_key }
    }
}

impl<S, B> Transform<S, ServiceRequest> for UserValidator
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = UserValidatorService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        let (validation, decoder) = types::jwt_ess::decoding_key(&self.secret_key);

        ready(Ok(UserValidatorService {
            service,
            validation,
            decoder,
        }))
    }
}

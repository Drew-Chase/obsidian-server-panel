use actix_web::dev::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, HttpMessage};
use futures_util::future::{ok, Ready};
use log::{debug, error, info, warn};
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;

/// Middleware for extracting and logging authentication tokens from requests.
pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    /// Initialize the middleware transformation.
    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddlewareService {
            service: Rc::new(service),
        })
    }
}

/// Service for `AuthMiddleware` to handle authentication token extraction and logging.
pub struct AuthMiddlewareService<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    /// Polls readiness of the inner service.
    fn poll_ready(
        &self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        debug!("Polling service readiness");
        self.service.poll_ready(cx)
    }

    /// Handles the incoming request, extracts the authentication token if present, and logs relevant information.
    fn call(&self, req: ServiceRequest) -> Self::Future {
        let connection_info = req.connection_info().clone();
        let addr = connection_info
            .realip_remote_addr()
            .unwrap_or("Unknown IP Address")
            .to_string();
        let headers = req.headers().clone();

        let service = Rc::clone(&self.service);

        let fut = async move {
            if let Some(auth_header) = headers.get("X-Authorization-Token") {
                match auth_header.to_str() {
                    Ok(auth_token) => {
                        info!("Received auth token: {}", auth_token);
                        match authentication::validation::validate_token(auth_token, &addr) {
                            Ok(user) => {
                                info!("Token is valid");
                                req.extensions_mut().insert(user);
                            }
                            Err(e) => {
                                error!(
                                    "Connection attempted with invalid token: {} - '{}'\n{:?}",
                                    addr, auth_token, e
                                );
                            }
                        }
                    }
                    Err(e) => {
                        error!("Failed to parse auth token: {}", e);
                    }
                }
            } else {
                // This could either mean that the path is public or that the client is not authorized
                warn!(
                    "No auth token provided for request from '{}' to path '{}'",
                    addr,
                    req.path()
                );
            }

            service.call(req).await
        };

        Box::pin(fut)
    }
}

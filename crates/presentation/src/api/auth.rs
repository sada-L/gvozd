use actix_web::{HttpResponse, Responder, ResponseError, web};
use application::services::auth::AuthService;
use std::sync::Arc;

use crate::{
    api::{requests::auth::AuthSignupRequest, responses::auth::AuthResponse},
    errors::ApiError,
};

pub struct AuthController {
    service: Arc<AuthService>,
}

impl AuthController {
    pub fn new(service: Arc<AuthService>) -> Self {
        Self { service }
    }

    pub async fn signup(
        data: web::Data<AuthController>,
        req: web::Json<AuthSignupRequest>,
    ) -> impl Responder {
        match data.service.signup(req.into_inner().into()).await {
            Ok(res) => HttpResponse::Ok().json(AuthResponse::from(res)),
            Err(e) => ApiError::from(e).error_response(),
        }
    }
}

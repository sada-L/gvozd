use actix_web::web::{self, ServiceConfig};

use crate::api::auth;

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(web::resource("/health").route(web::get().to(|| async { "OK" })))
            .service(web::scope("/auth").service(
                web::resource("/signup").route(web::post().to(auth::AuthController::signup)),
            )),
    );
}

use std::time::Duration;

use actix_cors::Cors;
use actix_web::{App, HttpServer, dev::Server, http, middleware};
use presentation::routes;

use crate::app::container;

pub fn build_http_server(deps: &container::Dependency, addr: &str) -> std::io::Result<Server> {
    let auth_controller = deps.auth_controller.clone();
    Ok(HttpServer::new({
        move || {
            App::new()
                .app_data(auth_controller.clone())
                .wrap(middleware::Compress::default())
                .wrap(middleware::NormalizePath::trim())
                .wrap(middleware::Logger::default())
                .wrap(
                    Cors::default()
                        .allowed_origin("http://127.0.0.1:8080")
                        .allowed_origin("http://localhost:8080")
                        .allowed_origin("http://0.0.0.0:8080")
                        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                        .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                        .allowed_header(http::header::CONTENT_TYPE)
                        .max_age(3600),
                )
                .configure(routes::configure)
        }
    })
    .keep_alive(Duration::from_secs(75))
    .client_request_timeout(Duration::from_secs(60))
    .client_disconnect_timeout(Duration::from_secs(5))
    .bind(addr)?
    .run())
}

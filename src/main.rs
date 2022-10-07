use poem::{error::NotFoundError, http::StatusCode, listener::TcpListener, EndpointExt, Response, Route, Server};

mod features;
mod routes;
use routes::api::api_routes;
mod server;
use server::configuration::init_env_variables;
use server::logger::init_logger;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config = init_env_variables();
    let logger = init_logger();
    let app = Route::new().nest("/api", api_routes()).catch_error(|_: NotFoundError| async move {
        Response::builder().status(StatusCode::NOT_FOUND).body("Whoops! That route doesn't exist!")
    });
    match logger {
        Ok(logger) => logger.flush(),
        _ => log::trace!("Logger error"),
    }
    Server::new(TcpListener::bind(format!("{}:{}", config.host, config.port))).run(app).await
}

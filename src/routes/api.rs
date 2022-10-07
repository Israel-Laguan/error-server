use poem::Route;

mod status;
use status::status_routes;

pub fn api_routes() -> Route {
    Route::new().nest("/status", status_routes())
}

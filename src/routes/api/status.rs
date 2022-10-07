use poem::{get, Route};

mod controllers;
use controllers::{about, am_i_up, dependency};

pub fn status_routes() -> Route {
    Route::new()
        .at("/about", get(about))
        .at("/am-i-up", get(am_i_up))
        .at("/dependency/:dependency", get(dependency))
}

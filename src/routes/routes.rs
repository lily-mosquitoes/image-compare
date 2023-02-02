use yew::{
    html,
    Html,
};
use yew_router::{
    Routable,
    components::Redirect
};

use crate::pages;

#[derive(Clone, Routable, PartialEq)]
pub(crate) enum Route {
    #[at("/")]
    Root,
    #[at("/images")]
    Images,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub(crate) fn switch(routes: Route) -> Html {
    match routes {
        Route::Root => html! { <Redirect<Route> to={Route::Images} /> }
        Route::Images => html! { <pages::Images /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

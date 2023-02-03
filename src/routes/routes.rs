use yew::{
    html,
    Html,
};
use yew_router::{
    components::Redirect,
    Routable,
};

use crate::pages;

#[derive(Clone, Routable, PartialEq)]
pub(crate) enum Route {
    #[at("/")]
    Root,
    #[at("/images")]
    ImagesToCompare,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub(crate) fn switch(routes: Route) -> Html {
    match routes {
        Route::Root => {
            html! { <Redirect<Route> to={Route::ImagesToCompare} /> }
        },
        Route::ImagesToCompare => {
            html! { <pages::ImagesToCompare /> }
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

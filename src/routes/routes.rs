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
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub(crate) fn switch(routes: Route) -> Html {
    match routes {
        Route::Root => {
            html! { <pages::ImagesToCompare /> }
        },
        Route::NotFound => {
            html! { <Redirect<Route> to={Route::Root} /> }
        },
    }
}

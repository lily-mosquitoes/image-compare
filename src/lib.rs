pub(crate) mod dom;
pub(crate) mod pages;
pub(crate) mod request;
pub(crate) mod routes;
pub(crate) mod shared_components;

use yew::{
    classes,
    function_component,
    html,
    Html,
};
use yew_router::{
    router::BrowserRouter,
    switch::Switch,
};

use crate::routes::{
    switch,
    Route,
};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <section
            id={"main"}
            class={classes![
                "h-screen",
                "font-hyperlegible",
                "bg-gradient-to-tr",
                "from-stone-700",
                "via-stone-700",
                "to-stone-500",
            ]}
        >
            <BrowserRouter>
                    <Switch<Route> render={switch} />
            </BrowserRouter>
        </section>
    }
}

#[cfg(test)]
pub(crate) use macros_for_tests::*;

#[cfg(test)]
pub(crate) mod macros_for_tests {
    macro_rules! wasm_sleep {
        ($time_in_ms:literal) => {
            yew::platform::time::sleep(
                std::time::Duration::from_millis($time_in_ms),
            )
            .await;
        };
    }
    pub(crate) use wasm_sleep;

    macro_rules! render_yew_component {
        ($component:ident) => {
            yew::Renderer::<$component>::with_root(
                crate::dom::DOM::get_element_by_id("output")
                    .expect("element with id #output to be present"),
            )
            .render();

            crate::macros_for_tests::wasm_sleep!(150);
        };
    }
    pub(crate) use render_yew_component;
}

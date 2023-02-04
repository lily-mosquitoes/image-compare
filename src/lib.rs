pub(crate) mod dom;
pub(crate) mod pages;
pub(crate) mod request;
pub(crate) mod routes;
pub(crate) mod shared_components;

// use wasm_bindgen::JsCast;
// use web_sys;
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

            crate::macros_for_tests::wasm_sleep!(100);
        };
    }
    pub(crate) use render_yew_component;
}

#[cfg(test)]
pub(crate) use macros_for_tests::*;

// #[cfg(test)]
// mod tests {
//     use wasm_bindgen_test::{
//         wasm_bindgen_test,
//         wasm_bindgen_test_configure,
//     };
//
//     use super::{
//         App,
//         MAIN_SECTION_ID,
//     };
//     use crate::{
//         dom::DOM,
//         test_helpers::render_yew_component,
//     };
//
//     wasm_bindgen_test_configure!(run_in_browser);
//
//     #[wasm_bindgen_test]
//     async fn main_section_id_matches_static() {
//         render_yew_component!(App);
//
//         let first_section_id = DOM::get_element_by_id("output")
//             .expect("output to be rendered")
//             .first_element_child()
//             .expect("main section of body to be rendered")
//             .id();
//
//         assert_eq!(&first_section_id, MAIN_SECTION_ID);
//     }
// }

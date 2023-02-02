pub(crate) mod dom;
pub(crate) mod pages;
pub(crate) mod routes;
pub(crate) mod shared_components;
pub(crate) mod yew_tester;

// use wasm_bindgen::JsCast;
// use web_sys;
use yew::{
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

pub(crate) static MAIN_SECTION_ID: &str = "main";

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <section
                id={MAIN_SECTION_ID}
                class="h-screen font-hyperlegible bg-gradient-to-tr from-stone-700 via-stone-700 to-stone-500"
            >
                <Switch<Route> render={switch} />
            </section>
        </BrowserRouter>
    }
}

// #[cfg(test)]
// mod test {
//     use wasm_bindgen_test::{
//         wasm_bindgen_test,
//         wasm_bindgen_test_configure,
//     };
//
//     use super::App;
//
//     wasm_bindgen_test_configure!(run_in_browser);
//
//     #[wasm_bindgen_test]
//     fn for_new_user_show_instructions() {
//         yew::Renderer::<App>::new().render();
//     }
//
//     fn for_old_user_do_not_show_instructions() {}
// }

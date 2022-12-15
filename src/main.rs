use yew::prelude::*;
use yew_router::prelude::*;
use wasm_bindgen::JsCast;
use web_sys;

mod pages;

#[derive(Clone, Routable, PartialEq)]
pub(crate) enum Route {
    #[at("/")]
    Welcome,
    #[at("/compare")]
    Compare,
    #[at("/success")]
    Success,
    #[at("/failure")]
    Failure,
}

fn htmldocument() -> web_sys::HtmlDocument {
    web_sys::window()
        .expect("window to be present")
        .document()
        .expect("document to be present") 
        .dyn_into::<web_sys::HtmlDocument>()
        .expect("Document to be castable to HtmlDocument")
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Welcome => {
            let raw_cookies = htmldocument().cookie().ok();
    
            let fingerprint_exists = match raw_cookies {
                Some(cookies_str) => cookies_str.contains("fingerprint"),
                None => false,
            };

            if fingerprint_exists {
                html! {
                    <Redirect<Route> to={Route::Compare} />
                }
            } else {
                html! { <pages::Welcome /> }
            }
        },
        Route::Compare => html! { <pages::Compare /> },
        Route::Success => html! { <h1>{ "Success" }</h1> },
        Route::Failure => html! { <h1>{ "Failure" }</h1> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <section id="main">
                <Switch<Route> render={switch} />
            </section>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

#[cfg(test)]
pub(crate) mod tests_setup;

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::tests_setup::*;

    setup_environment!();

    fn unset_fingerprint() {
        WasmWindow::document().set_raw_cookies(EMPTY_FINGERPRINT);
    }

    fn fingerprint_exists() -> bool {
        if let Some(cookies) = WasmWindow::document().get_raw_cookies() {
            cookies.contains("fingerprint")
        } else {
            false
        }
    }

    #[wasm_bindgen_test]
    async fn test_first_render_and_fingerprint_redirect() {
        unset_fingerprint();
        assert!(!fingerprint_exists());

        render_app!(App);

        let mut id_of_first_child_from_main = WasmWindow::document()
            .get_element_by_id("main")
            .first_element_child()
            .expect("child to be present")
            .id();
        assert_eq!(id_of_first_child_from_main.as_str(), "welcome");

        let fingerprint_button = WasmWindow::document()
            .get_element_by_id("get_fingerprint")
            .as_html();

        fingerprint_button.click();
        wait_for_render!();
        assert!(fingerprint_exists());

        id_of_first_child_from_main = WasmWindow::document()
            .get_element_by_id("main")
            .first_element_child()
            .expect("child to be present")
            .id();
        assert_eq!(id_of_first_child_from_main.as_str(), "compare");
    }
}


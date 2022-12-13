use yew::prelude::*;
use yew_router::prelude::*;
use wasm_bindgen::JsCast;
use web_sys;

mod pages;

#[derive(Clone, Routable, PartialEq)]
enum Route {
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
pub mod tests {
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    use super::*;
    use wasm_bindgen_test::*;
    use std::time::Duration;
    use yew::platform::time::sleep;

    fn document() -> web_sys::Document {
        web_sys::window()
            .expect("window to be present")
            .document()
            .expect("document to be present") 
    }

    fn get_element_by_id(id: &str) -> web_sys::Element {
        document()
            .get_element_by_id(id)
            .expect("element with id to be present")
    }

    fn htmldocument() -> web_sys::HtmlDocument {
        document()
            .dyn_into::<web_sys::HtmlDocument>()
            .expect("Document to be castable to HtmlDocument")
    }

    fn cookie_exists(name: &str) -> bool {
        if let Some(cookies) = htmldocument().cookie().ok() {
            cookies.contains(name)
        } else {
            false
        }
    }

    macro_rules! render_app {
        () => {
            yew::Renderer::<App>::with_root(get_element_by_id("output"))
                .render();
            // wait for rendering
            sleep(Duration::from_millis(100)).await;
        }
    }

    #[wasm_bindgen_test]
    async fn test_fingerprint_absent() {
        let _ = htmldocument()
            .set_cookie("fingerprint=; \
                        expires=Thu, 01 Jan 1970 00:00:00 UTC; \
                        path=/;")
            .expect("cookie to be unset");

        assert!(!cookie_exists("fingerprint"));

        render_app!();

        let mut id_of_first_child_from_main = get_element_by_id("main")
            .first_element_child()
            .expect("child to be present")
            .id();
        assert_eq!(id_of_first_child_from_main.as_str(), "welcome");

        let _ = htmldocument()
            .set_cookie("fingerprint=testvalue; path=/;")
            .expect("cookie to be set");

        assert!(cookie_exists("fingerprint"));

        render_app!();

        id_of_first_child_from_main = get_element_by_id("main")
            .first_element_child()
            .expect("child to be present")
            .id();
        assert_eq!(id_of_first_child_from_main.as_str(), "compare");
    }
}


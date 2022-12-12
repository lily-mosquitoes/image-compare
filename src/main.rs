use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/compare")]
    Compare,
    #[at("/success")]
    Success,
    #[at("/failure")]
    Failure,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <section id="main"><h1>{ "Hello" }</h1></section> },
        Route::Compare => html! { <h1>{ "Compare" }</h1> },
        Route::Success => html! { <h1>{ "Success" }</h1> },
        Route::Failure => html! { <h1>{ "Failure" }</h1> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

#[cfg(test)]
pub mod tests {
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    use super::*;
    use wasm_bindgen_test::*;
    use wasm_bindgen::JsCast;
    use web_sys;
    use std::time::Duration;
    use yew::platform::time::sleep;

    fn document() -> web_sys::Document {
        web_sys::window()
            .expect("window to be present")
            .document()
            .expect("document to be present") 
    }

    fn get_inner_html_by_id(id: &str) -> String {
        document()
            .get_element_by_id(id)
            .expect("element with id to be present")
            .inner_html()
    }

    fn htmldocument() -> web_sys::HtmlDocument {
        document()
            .dyn_into::<web_sys::HtmlDocument>()
            .expect("Document to be castable to HtmlDocument")
    }

    fn get_cookies() -> Option<String> {
        htmldocument()
            .cookie()
            .ok()
    }

    fn cookie_exists(name: &str) -> bool {
        if let Some(cookies) = get_cookies() {
            cookies.contains(name)
        } else {
            false
        }
    }

    #[wasm_bindgen_test]
    async fn test_cookies_absent() {
        yew::Renderer::<App>::with_root(
            document().get_element_by_id("output").unwrap(),
        )
        .render();

        // wait for rendering
        sleep(Duration::from_millis(100)).await;

        let _ = htmldocument()
            .set_cookie("fingerprint=; \
                        expires=Thu, 01 Jan 1970 00:00:00 UTC; \
                        path=/;");

        assert!(!cookie_exists("fingerprint"));

        let result = get_inner_html_by_id("main");
        assert_eq!(result.as_str(), "<h1>Hello</h1>");
    }

    #[wasm_bindgen_test]
    async fn test_cookies_present() {
        yew::Renderer::<App>::with_root(
            document().get_element_by_id("output").unwrap(),
        )
        .render();

        // wait for rendering
        sleep(Duration::from_millis(100)).await;

        let _ = htmldocument().set_cookie("fingerprint=testvalue");

        assert!(cookie_exists("fingerprint"));

        let result = get_inner_html_by_id("main");        
        assert_eq!(result.as_str(), "<h1>Hello</h1>");
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

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
    use gloo_utils;
    use std::time::Duration;
    use wasm_bindgen_test::*;
    use yew::platform::time::sleep;

    fn get_inner_html_by_id(id: &str) -> String {
        gloo_utils::document()
            .get_element_by_id(id)
            .expect("element with id to be present")
            .inner_html()
    }

    #[wasm_bindgen_test]
    async fn test_wasm() {
        yew::Renderer::<App>::with_root(
            gloo_utils::document().get_element_by_id("output").unwrap(),
        )
        .render();

        // wait for rendering
        sleep(Duration::from_millis(100)).await;

        let result = get_inner_html_by_id("main");
        assert_eq!(result.as_str(), "<h1>Hello</h1>");
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

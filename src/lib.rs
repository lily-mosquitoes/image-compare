pub(crate) mod assets;
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
pub(crate) use helpers_for_tests::*;

#[cfg(test)]
mod helpers_for_tests {
    pub(crate) fn markdown_to_decoded_html(text: &str) -> String {
        let html = markdown::to_html(text);
        // insert target="_blank" on links
        let html =
            html.replace("<a href", "<a target=\"_blank\" href");
        // new lines between elements do not get rendered to the DOM
        let html = html.trim().replace(">\n<", "><");
        // encoded characters are escaped when rendered to the DOM
        html_escape::decode_html_entities(&html).into_owned()
    }

    pub(crate) async fn wasm_sleep_in_ms(amount: u64) {
        let duration = std::time::Duration::from_millis(amount);
        yew::platform::time::sleep(duration).await;
    }

    macro_rules! render_yew_component {
        ($component:ident) => {
            yew::Renderer::<$component>::with_root(
                crate::dom::DOM::get_element_by_id("output")
                    .expect("element with id #output to be present"),
            )
            .render();
        };
    }
    pub(crate) use render_yew_component;
}

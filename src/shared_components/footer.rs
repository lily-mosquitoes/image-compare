use yew::{
    classes,
    function_component,
    html,
    Children,
    Html,
    Properties,
};

use crate::shared_components::Button;

#[derive(Properties, PartialEq, Default)]
pub(crate) struct FooterProps {
    pub(crate) children: Children,
}

#[function_component(Footer)]
pub(crate) fn footer(props: &FooterProps) -> Html {
    html! {
        <footer
            id="footer"
            class={classes![
                "bg-stone-800",
                "p-12",
                "shrink-0",
                "flex",
                "flex-row",
                "justify-between",
            ]}
        >
            <Button
                id={"select_language_button"}
                class={classes![
                    "text-gray-300",
                    "border-2",
                    "border-gray-300",
                ]}
            >
                { "EN" }
            </Button>
            {for props.children.iter()}
        </footer>
    }
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::{
        wasm_bindgen_test,
        wasm_bindgen_test_configure,
    };

    use super::Footer;
    use crate::{
        dom::DOM,
        render_yew_component,
        wasm_sleep_in_ms,
    };
    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn select_language_button_exists() {
        render_yew_component!(Footer);
        wasm_sleep_in_ms(50).await;

        assert!(
            DOM::get_button_by_id("select_language_button").is_some()
        );
    }
}

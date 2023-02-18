use yew::{
    classes,
    function_component,
    html,
    Children,
    Html,
    Properties,
};

use crate::shared_components::LanguageButton;

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
                "fixed",
                "lg:relative",
                "bottom-0",
                "shrink-0",
                "w-full",
                "bg-stone-800",
                "p-12",
                "lg:p-2",
                "flex",
                "flex-row",
                "justify-between",
                "items-center",
            ]}
        >
            <LanguageButton />
            <a href="https://github.com/lily-mosquitoes">
                <p class={classes![
                    "text-3xl",
                    "lg:text-base",
                    "text-gray-500"
                ]}>
                    { "Made by Lílian" }
                </p>
            </a>
            {for props.children.iter()}
        </footer>
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::Ordering;

    use wasm_bindgen::JsCast;
    use wasm_bindgen_test::{
        wasm_bindgen_test,
        wasm_bindgen_test_configure,
    };

    use super::Footer;
    use crate::{
        dom::DOM,
        render_yew_component,
        wasm_sleep_in_ms,
        AVAILABLE_LANGUAGES,
        DEFAULT_LANGUAGE,
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

    #[wasm_bindgen_test]
    async fn select_language_button_shows_current_language() {
        // add 1 to len to run even if no languages are available
        for language_index in 0..AVAILABLE_LANGUAGES.len() + 1 {
            DEFAULT_LANGUAGE.store(language_index, Ordering::SeqCst);

            render_yew_component!(Footer);
            wasm_sleep_in_ms(50).await;

            let button =
                DOM::get_button_by_id("select_language_button")
                    .expect(
                        "Element #select_language_button to exist",
                    );

            let expected =
                if AVAILABLE_LANGUAGES.len() > language_index {
                    format!(
                        "{}",
                        AVAILABLE_LANGUAGES[language_index].display()
                    )
                } else if AVAILABLE_LANGUAGES.len() > 0 {
                    format!("{}", AVAILABLE_LANGUAGES[0].display())
                } else {
                    "".to_string()
                };

            assert_eq!(button.inner_html(), expected);
        }
    }

    #[wasm_bindgen_test]
    async fn language_menu_is_hidden_by_default() {
        render_yew_component!(Footer);
        wasm_sleep_in_ms(50).await;

        assert!(DOM::get_element_by_id("language_menu").is_none())
    }

    #[wasm_bindgen_test]
    async fn select_language_button_shows_language_menu() {
        render_yew_component!(Footer);
        wasm_sleep_in_ms(50).await;

        let button = DOM::get_button_by_id("select_language_button")
            .expect("Element #select_language_button to exist")
            .dyn_into::<web_sys::HtmlElement>()
            .expect("Element to be castable to HtmlElement");

        button.click();
        wasm_sleep_in_ms(50).await; // allow page to re-render

        assert!(DOM::get_element_by_id("language_menu").is_some());
    }

    #[wasm_bindgen_test]
    async fn select_language_button_hides_language_menu_if_open() {
        render_yew_component!(Footer);
        wasm_sleep_in_ms(50).await;

        let button = DOM::get_button_by_id("select_language_button")
            .expect("Element #select_language_button to exist")
            .dyn_into::<web_sys::HtmlElement>()
            .expect("Element to be castable to HtmlElement");

        button.click();
        wasm_sleep_in_ms(50).await; // allow page to re-render

        button.click();
        wasm_sleep_in_ms(50).await; // allow page to re-render

        assert!(DOM::get_element_by_id("language_menu").is_none());
    }

    #[wasm_bindgen_test]
    async fn language_menu_contains_available_languages() {
        render_yew_component!(Footer);
        wasm_sleep_in_ms(50).await;

        let button = DOM::get_button_by_id("select_language_button")
            .expect("Element #select_language_button to exist")
            .dyn_into::<web_sys::HtmlElement>()
            .expect("Element to be castable to HtmlElement");

        button.click();
        wasm_sleep_in_ms(50).await; // allow page to re-render

        let menu_items = DOM::get_element_by_id("language_menu")
            .expect("Element #language_menu to exist")
            .inner_html();

        for language in AVAILABLE_LANGUAGES.iter() {
            assert!(menu_items
                .contains(&format!("<p>{}</p>", language.display())))
        }
    }

    #[wasm_bindgen_test]
    async fn language_menu_buttons_change_current_language() {
        render_yew_component!(Footer);
        wasm_sleep_in_ms(50).await;

        for language in AVAILABLE_LANGUAGES.iter() {
            let button =
                DOM::get_button_by_id("select_language_button")
                    .expect(
                        "Element #select_language_button to exist",
                    )
                    .dyn_into::<web_sys::HtmlElement>()
                    .expect("Element to be castable to HtmlElement");

            button.click();
            wasm_sleep_in_ms(50).await; // allow page to re-render

            let id = format!("{}", language.display());
            let menu_button = DOM::get_button_by_id(&id)
                .expect(&format!("Element #{} to exist", &id))
                .dyn_into::<web_sys::HtmlElement>()
                .expect("Element to be castable to HtmlElement");

            menu_button.click();
            wasm_sleep_in_ms(50).await; // allow page to re-render

            assert_eq!(button.inner_html(), id);
        }
    }
}

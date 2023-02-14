use std::sync::atomic::Ordering;

use yew::{
    classes,
    function_component,
    html,
    Children,
    Html,
    Properties,
};

use crate::{
    shared_components::Button,
    AVAILABLE_LANGUAGES,
    SELECTED_LANGUAGE,
};

#[derive(Properties, PartialEq, Default)]
pub(crate) struct FooterProps {
    pub(crate) children: Children,
}

#[function_component(Footer)]
pub(crate) fn footer(props: &FooterProps) -> Html {
    let available_languages: Vec<String> = AVAILABLE_LANGUAGES
        .iter()
        .map(|x| format!("{}", x.display()))
        .collect();

    let index = SELECTED_LANGUAGE.load(Ordering::SeqCst);

    let selected_language = match (
        AVAILABLE_LANGUAGES.len() > 0,
        AVAILABLE_LANGUAGES.len() > index,
    ) {
        (true, true) => available_languages[index].clone(),
        (true, false) => available_languages[0].clone(),
        (false, _) => "".to_string(),
    };

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
                { selected_language }
            </Button>
            {for props.children.iter()}
        </footer>
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::Ordering;

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
        SELECTED_LANGUAGE,
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
    async fn select_language_button_shows_selected_language() {
        // add 1 to len to run even if no languages are available
        for selected_language in 0..AVAILABLE_LANGUAGES.len() + 1 {
            SELECTED_LANGUAGE
                .store(selected_language, Ordering::SeqCst);

            render_yew_component!(Footer);
            wasm_sleep_in_ms(50).await;

            let button =
                DOM::get_button_by_id("select_language_button")
                    .expect(
                        "Element #select_language_button to exist",
                    );

            let index = selected_language;
            let expected = if AVAILABLE_LANGUAGES.len() > index {
                format!("{}", AVAILABLE_LANGUAGES[index].display())
            } else if AVAILABLE_LANGUAGES.len() > 0 {
                format!("{}", AVAILABLE_LANGUAGES[0].display())
            } else {
                "".to_string()
            };

            assert_eq!(button.inner_html(), expected);
        }
    }
}

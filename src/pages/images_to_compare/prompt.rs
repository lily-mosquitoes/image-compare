use yew::{
    classes,
    function_component,
    html,
    use_context,
    Html,
    UseReducerHandle,
};

use crate::{
    pages::markdown_to_yew_html,
    Language,
};

#[function_component(Prompt)]
pub(super) fn prompt() -> Html {
    let language = match use_context::<UseReducerHandle<Language>>() {
        Some(ctx) => (*ctx).clone(),
        None => Language::default(),
    };

    let which_is_best_prompt = language.load_file("which_is_best_prompt.md");
    let which_is_best_prompt =
        markdown_to_yew_html(which_is_best_prompt.unwrap_or(""));

    html! {
        <section
            id="which_is_best_prompt"
            class={classes![
                "self-center",
                "pt-4",
                "text-xl",
                "text-gray-200"
            ]}
        >
            { which_is_best_prompt }
        </section>
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::Ordering;

    use wasm_bindgen_test::{
        wasm_bindgen_test,
        wasm_bindgen_test_configure,
    };

    use super::Prompt;
    use crate::{
        dom::DOM,
        markdown_to_decoded_html,
        render_yew_component,
        wasm_sleep_in_ms,
        Language,
        AVAILABLE_LANGUAGES,
        DEFAULT_LANGUAGE,
    };
    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn which_is_best_prompt_markdown_exists() {
        // add 1 to len to run even if no languages are available
        for language_index in 0..AVAILABLE_LANGUAGES.len() + 1 {
            let language = Language {
                index: language_index,
            };
            let file = language.load_file("which_is_best_prompt.md");

            assert!(file.is_some())
        }
    }

    #[wasm_bindgen_test]
    async fn which_is_best_prompt_text_is_visible() {
        // add 1 to len to run even if no languages are available
        for language_index in 0..AVAILABLE_LANGUAGES.len() + 1 {
            DEFAULT_LANGUAGE.store(language_index, Ordering::SeqCst);

            render_yew_component!(Prompt);
            wasm_sleep_in_ms(50).await;

            let language = Language::default();
            let expected = language.load_file("which_is_best_prompt.md");
            let expected = markdown_to_decoded_html(expected.unwrap_or(""));

            let text = DOM::get_element_by_id("which_is_best_prompt")
                .expect("Element #which_is_best_prompt to exist");

            assert_eq!(text.inner_html(), expected);
        }
    }
}

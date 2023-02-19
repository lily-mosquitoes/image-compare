use std::sync::atomic::Ordering;

use yew::{
    classes,
    function_component,
    html,
    use_context,
    use_effect,
    use_state_eq,
    Callback,
    Html,
};

use crate::{
    dom::{
        console_error,
        DOM,
    },
    shared_components::Button,
    Language,
    LanguageContext,
    AVAILABLE_LANGUAGES,
    DEFAULT_LANGUAGE,
};

#[function_component(LanguageButton)]
pub(crate) fn language_button() -> Html {
    let language = use_context::<LanguageContext>();
    let language_menu_visible = use_state_eq(|| false);

    let toggle_language_menu = {
        let language_menu_visible = language_menu_visible.clone();
        Callback::from(move |_| {
            language_menu_visible.set(!*language_menu_visible);
        })
    };

    let change_language = |language_index| {
        let language = language.clone();
        let language_menu_visible = language_menu_visible.clone();
        Callback::from(move |_| {
            match language.clone() {
                Some(ctx) => ctx.dispatch(language_index),
                None => DEFAULT_LANGUAGE
                    .store(language_index, Ordering::SeqCst),
            }
            language_menu_visible.set(false);
        })
    };

    let available_languages: Vec<String> = AVAILABLE_LANGUAGES
        .iter()
        .map(|x| format!("{}", x.display()))
        .collect();

    let language_index = match language.clone() {
        Some(ctx) => ctx.index,
        None => Language::default().index,
    };

    let selected_language = match (
        AVAILABLE_LANGUAGES.len() > 0,
        AVAILABLE_LANGUAGES.len() > language_index,
    ) {
        (true, true) => available_languages[language_index].clone(),
        (true, false) => available_languages[0].clone(),
        (false, _) => "".to_string(),
    };

    {
        let selected_language = selected_language.to_lowercase();
        use_effect(move || {
            match DOM::set_document_language(&selected_language) {
                Ok(_) => (),
                Err(error) => console_error!(error),
            }
        });
    }

    html! {
        <>
            if *language_menu_visible {
                <section
                    id={"language_menu"}
                    class={classes![
                        "fixed",
                        "z-10",
                        "bottom-20",
                    ]}
                >
                    <section
                        id={"language_menu_items"}
                        class={classes![
                            "bg-stone-800",
                            "drop-shadow-2xl",
                            "p-4",
                            "rounded-xl",
                            "flex",
                            "flex-col",
                            "gap-2",
                        ]}
                    >
                        {
                            available_languages
                                .iter()
                                .enumerate()
                                .map(|(i, l)| html! {
                                    <Button
                                        id={l.clone()}
                                        key={l.clone()}
                                        class={classes![
                                            "text-xl",
                                            "text-gray-300",
                                            "aspect-square",
                                        ]}
                                        onclick={change_language(i)}
                                    >
                                        <p>{ l }</p>
                                    </Button>
                                })
                                .collect::<Html>()
                        }
                    </section>
                </section>
            }
            <Button
                id={"select_language_button"}
                class={classes![
                    "text-gray-300",
                    "border-2",
                    "border-gray-300",
                    "w-[2.7em]",
                    "aspect-square",
                ]}
                onclick={toggle_language_menu}
            >
                { selected_language }
            </Button>
        </>
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

    use super::LanguageButton;
    use crate::{
        dom::DOM,
        render_yew_component,
        wasm_sleep_in_ms,
        AVAILABLE_LANGUAGES,
        DEFAULT_LANGUAGE,
    };
    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn document_language_matches_current_language() {
        // add 1 to len to run even if no languages are available
        for language_index in 0..AVAILABLE_LANGUAGES.len() + 1 {
            DEFAULT_LANGUAGE.store(language_index, Ordering::SeqCst);
            let current_language = if AVAILABLE_LANGUAGES.len() == 0 {
                "".to_string()
            } else if language_index >= AVAILABLE_LANGUAGES.len() {
                format!("{}", AVAILABLE_LANGUAGES[0].display())
            } else {
                format!(
                    "{}",
                    AVAILABLE_LANGUAGES[language_index].display()
                )
            };

            render_yew_component!(LanguageButton);
            wasm_sleep_in_ms(50).await;

            let html_document = DOM::document()
                .expect("Document to exist")
                .document_element()
                .expect("Document Element to exist")
                .dyn_into::<web_sys::HtmlElement>()
                .expect("Element to be castable to HtmlElement");

            assert_eq!(
                html_document.lang(),
                current_language.to_lowercase()
            );
        }
    }

    #[wasm_bindgen_test]
    async fn changing_language_changes_lang_attribute_on_document() {
        render_yew_component!(LanguageButton);
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

            let html_document = DOM::document()
                .expect("Document to exist")
                .document_element()
                .expect("Document Element to exist")
                .dyn_into::<web_sys::HtmlElement>()
                .expect("Element to be castable to HtmlElement");

            assert_eq!(html_document.lang(), id.to_lowercase());
        }
    }
}

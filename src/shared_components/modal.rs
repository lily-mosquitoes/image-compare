use std::path::PathBuf;

use yew::{
    classes,
    create_portal,
    function_component,
    html,
    use_context,
    Callback,
    Children,
    Html,
    Properties,
};

use crate::{
    assets::XMark,
    dom::DOM,
    load_file_from_language,
    pages::markdown_to_yew_html,
    shared_components::Button,
    Language,
    LanguageContext,
};

#[derive(Properties, PartialEq)]
pub(crate) struct ModalProps {
    pub(crate) id: String,
    pub(crate) onclose: Callback<()>,
    pub(crate) children: Children,
}

#[function_component]
pub(crate) fn Modal(props: &ModalProps) -> Html {
    let language = match use_context::<LanguageContext>() {
        Some(ctx) => (*ctx).clone(),
        None => Language::default(),
    };

    let close_modal_button_sr = load_file_from_language(
        PathBuf::from("close_modal_button_sr.md"),
        language.index,
    );
    let close_modal_button_sr =
        markdown_to_yew_html(close_modal_button_sr.unwrap_or(""));

    let modal_host = DOM::body_first_element_child()
        .expect("first section of body to be rendered");

    let close_modal = {
        let event = props.onclose.clone();
        Callback::from(move |_| event.emit(()))
    };

    create_portal(
        html! {
            <section
                id={props.id.clone()}
                class={classes![
                    "bg-black/[0.4]",
                    "w-full",
                    "h-full",
                    "fixed",
                    "top-0",
                    "left-0",
                    "z-10",
                ]}
            >
                <section
                    id="modal_content"
                    class={classes![
                        "mt-8",
                        "lg:mt-8",
                        "mx-auto",
                        "w-4/5",
                        "lg:w-1/3",
                        "rounded-xl",
                        "bg-stone-200",
                        "drop-shadow-2xl",
                    ]}
                >
                    <section
                        id="modal_header"
                        class={classes![
                            "flex",
                            "justify-end",
                            "px-4",
                            "pt-4",
                        ]}
                    >
                        <Button
                            id={format!(
                                "close_{}_button",
                                props.id.clone()
                            )}
                            onclick={close_modal}
                        >
                            <XMark
                                class={classes![
                                    "h-8",
                                    "stroke-black",
                                ]}
                            />
                            <span class={classes!["sr-only"]}>
                                { close_modal_button_sr }
                            </span>
                        </Button>
                    </section>
                    <section
                        id="modal_body"
                        class={classes![
                            "flex",
                            "flex-col",
                            "px-8",
                            "py-0",
                        ]}
                    >
                        {for props.children.iter()}
                    </section>
                </section>
            </section>
        },
        modal_host.into(),
    )
}

#[cfg(test)]
mod tests {
    use std::{
        path::PathBuf,
        sync::atomic::Ordering,
    };

    use wasm_bindgen_test::{
        wasm_bindgen_test,
        wasm_bindgen_test_configure,
    };
    use yew::{
        function_component,
        html,
        Html,
    };

    use super::Modal;
    use crate::{
        dom::DOM,
        helpers_for_tests::markdown_to_decoded_html,
        load_file_from_language,
        render_yew_component,
        wasm_sleep_in_ms,
        AVAILABLE_LANGUAGES,
        DEFAULT_LANGUAGE,
    };
    wasm_bindgen_test_configure!(run_in_browser);

    #[function_component(TestModal)]
    fn test_modal() -> Html {
        html! {
            <div>
                <Modal id={"test_modal"} onclose={|_| ()}>
                    { "test" }
                </Modal>
            </div>
        }
    }

    #[wasm_bindgen_test]
    async fn close_modal_button_exists() {
        render_yew_component!(TestModal);
        wasm_sleep_in_ms(50).await;

        assert!(DOM::get_button_by_id("close_test_modal_button")
            .is_some());
    }

    #[wasm_bindgen_test]
    fn close_modal_button_sr_markdown_exists() {
        // add 1 to len to run even if no languages are available
        for language_index in 0..AVAILABLE_LANGUAGES.len() + 1 {
            let file = load_file_from_language(
                PathBuf::from("close_modal_button_sr.md"),
                language_index,
            );

            assert!(file.is_some())
        }
    }

    #[wasm_bindgen_test]
    async fn close_modal_button_sr_text_is_rendered() {
        // add 1 to len to run even if no languages are available
        for language_index in 0..AVAILABLE_LANGUAGES.len() + 1 {
            DEFAULT_LANGUAGE.store(language_index, Ordering::SeqCst);

            render_yew_component!(TestModal);
            wasm_sleep_in_ms(50).await;

            let expected = load_file_from_language(
                PathBuf::from("close_modal_button_sr.md"),
                language_index,
            );
            let expected =
                markdown_to_decoded_html(expected.unwrap_or(""));

            let text =
                DOM::get_element_by_id("close_test_modal_button")
                    .expect(
                        "Element #close_test_modal_button to exist",
                    )
                    .last_element_child()
                    .expect("Last element child to exist");

            assert_eq!(text.inner_html(), expected);
        }
    }
}

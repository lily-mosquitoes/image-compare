use yew::{
    classes,
    create_portal,
    function_component,
    html,
    Callback,
    Children,
    Html,
    Properties,
};

use crate::{
    assets::XMark,
    dom::DOM,
    shared_components::Button,
};

#[derive(Properties, PartialEq)]
pub(crate) struct ModalProps {
    pub(crate) id: String,
    pub(crate) onclose: Callback<()>,
    pub(crate) children: Children,
}

#[function_component]
pub(crate) fn Modal(props: &ModalProps) -> Html {
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
                        "mt-24",
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
                            "px-8",
                            "lg:px-4",
                            "pt-8",
                            "lg:pt-4",
                        ]}
                    >
                        <Button
                            id={format!(
                                "close_{}_button",
                                props.id.clone()
                            )}
                            // class={classes![
                            //     "p-8",
                            //     "lg:p-3",
                            // ]}
                            onclick={close_modal}
                        >
                            <XMark
                                class={classes![
                                    "h-16",
                                    "lg:h-8",
                                    "stroke-black",
                                ]}
                            />
                            <span class={classes!["sr-only"]}>
                                { "Close" }
                            </span>
                        </Button>
                    </section>
                    <section
                        id="modal_body"
                        class={classes![
                            "flex",
                            "flex-col",
                            "px-8",
                            "py-8",
                            "lg:py-0",
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
        render_yew_component,
        wasm_sleep_in_ms,
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
}

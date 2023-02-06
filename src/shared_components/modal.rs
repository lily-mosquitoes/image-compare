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
                        "mx-auto",
                        "w-4/5",
                        "rounded-xl",
                        "bg-stone-200",
                        "border-4",
                        "border-white",
                        "drop-shadow-2xl",
                    ]}
                >
                    <section
                        id="modal_header"
                        class={classes![
                            "flex",
                            "justify-end",
                            "p-8",
                        ]}
                    >
                        <Button
                            id={"close_modal_button"}
                            onclick={close_modal}
                        >
                            { "X" }
                        </Button>
                    </section>
                    <section
                        id="modal_body"
                        class={classes!["flex", "flex-col", "p-8"]}
                    >
                        {for props.children.iter()}
                    </section>
                </section>
            </section>
        },
        modal_host.into(),
    )
}

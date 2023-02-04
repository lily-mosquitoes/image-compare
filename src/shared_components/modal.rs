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
    MAIN_SECTION_ID,
};

pub(crate) fn open_modal(id: &str) -> Callback<()> {
    let id = id.to_owned();

    Callback::from(
        move |_| match DOM::remove_class_from_element_by_id(
            "hidden", &id,
        ) {
            Ok(_) => (),
            Err(error) => web_sys::console::error_1(&error),
        },
    )
}

pub(crate) fn close_modal(id: &str) -> Callback<()> {
    let id = id.to_owned();

    Callback::from(move |_| {
        match DOM::add_class_to_element_by_id("hidden", &id) {
            Ok(_) => (),
            Err(error) => web_sys::console::error_1(&error),
        }
    })
}

#[derive(Properties, PartialEq)]
pub(crate) struct ModalProps {
    pub(crate) id: String,
    #[prop_or_default]
    pub(crate) children: Children,
}

#[function_component]
pub(crate) fn Modal(props: &ModalProps) -> Html {
    let modal_host = DOM::get_element_by_id(MAIN_SECTION_ID).expect(
        &format!("Expected to find a #{} element", MAIN_SECTION_ID),
    );

    let close_self = {
        let event = close_modal(&props.id.clone()).clone();
        Callback::from(move |_| event.emit(()))
    };

    create_portal(
        html! {
            <section
                id={props.id.clone()}
                class={classes![
                    "hidden",
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
                        <button
                            class={classes!["text-5xl", "drop-shadow-2xl"]}
                            onclick={close_self}
                        >
                            { "X" }
                        </button>
                    </section>
                    <section
                        id="modal_body"
                        class={classes!["flex", "p-8"]}
                    >
                        {for props.children.iter()}
                    </section>
                </section>
            </section>
        },
        modal_host.into(),
    )
}

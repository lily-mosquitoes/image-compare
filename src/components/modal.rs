use yew::prelude::*;
use image_compare::wasmjs;

#[derive(Properties, PartialEq)]
pub struct ModalProps {
    pub id: String,
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub(crate) fn Modal(props: &ModalProps) -> Html {
    let modal_host = wasmjs::get_element_by_id("main")
        .expect("Expected to find a #main element");

    let close_modal = {
        let id = props.id.clone();
        Callback::from(move |_| {
            match wasmjs::add_class_to_element_by_id("hidden", &id) {
                Ok(_) => (),
                Err(error) => web_sys::console::error_1(&error),
            }
        })
    };

    create_portal(
        html!{
            <section
                id={props.id.clone()}
                class="hidden bg-black/[0.4] w-full h-full \
                fixed top-0 left-0 z-10"
            >
                <section
                    id="modal_content"
                    class="mt-24 mx-auto w-4/5 rounded-xl \
                    bg-stone-200 border-4 border-white drop-shadow-2xl"
                >
                    <section
                        id="modal_header"
                        class="flex justify-end p-8"
                    >
                        <button
                            class="text-5xl drop-shadow-2xl"
                            onclick={close_modal}
                        >
                            { "X" }
                        </button>
                    </section>
                    <section
                        id="modal_body"
                        class="flex p-8"
                    >
                        {for props.children.iter()}
                    </section>
                </section>
            </section>
        },
        modal_host.into(),
    )
}



use yew::prelude::*;
use image_compare::wasmjs;
use crate::components::Modal;

#[derive(Properties, PartialEq)]
struct ChangeUserModalProps {
    id: String,
}

#[function_component(ChangeUserModal)]
fn change_user_modal(props: &ChangeUserModalProps) -> Html {
    html! {
        <Modal id={props.id.clone()}>
            <p class="text-5xl">{ "content" }</p>
        </Modal>
    }
}

#[function_component(Header)]
pub(super) fn header() -> Html {
    let change_user_modal_id = "change_user_modal".to_string();
    
    let open_change_user_modal = {
        let id = change_user_modal_id.clone();
        Callback::from(move |_| {
            match wasmjs::remove_class_from_element_by_id("hidden", &id) {
                Ok(_) => (),
                Err(error) => web_sys::console::error_1(&error),
            }
        })
    };

    html! {
        <section
            id="header"
            class="bg-stone-200 h-28 drop-shadow-2xl px-10 \
            flex flex-row justify-between"
        >
            <button
                class="text-5xl"
            >
                { "I'm done with 0 votes!" }
            </button>
            <button
                class="text-5xl"
                onclick={open_change_user_modal}
            >
                { "Change user" }
            </button>
            <ChangeUserModal id={change_user_modal_id}/>
        </section>
    }
}


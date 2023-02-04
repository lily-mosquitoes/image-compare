use yew::{
    classes,
    function_component,
    html,
    use_state,
    Callback,
    Html,
};

use super::change_user_modal::ChangeUserModal;
use crate::shared_components::Button;

#[function_component(Header)]
pub(super) fn header() -> Html {
    let change_user_modal_id = "change_user_modal".to_string();
    let change_user_modal_is_visible = use_state(|| false);

    let open_change_user_modal = {
        let change_user_modal_is_visible =
            change_user_modal_is_visible.clone();
        Callback::from(move |_| {
            change_user_modal_is_visible.set(true)
        })
    };

    let close_change_user_modal = {
        let change_user_modal_is_visible =
            change_user_modal_is_visible.clone();
        Callback::from(move |_| {
            change_user_modal_is_visible.set(false)
        })
    };

    html! {
        <section
            id="header"
            class={classes![
                "bg-stone-200",
                "h-28",
                "drop-shadow-2xl",
                "px-10",
                "flex",
                "flex-row",
                "justify-between",
            ]}
        >
            <Button
                text={ "I'm done with 0 votes!" }
                onclick={Callback::from(move |_| ())}
            />
            <Button
                text={ "Change user" }
                onclick={open_change_user_modal}
            />
            if *change_user_modal_is_visible {
                <ChangeUserModal
                    id={change_user_modal_id}
                    onclose={close_change_user_modal}
                />
            }
        </section>
    }
}

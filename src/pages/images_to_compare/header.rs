use yew::{
    classes,
    function_component,
    html,
    Callback,
    Html,
};

use super::change_user_modal::ChangeUserModal;
use crate::shared_components::{
    open_modal,
    Button,
};

#[function_component(Header)]
pub(super) fn header() -> Html {
    let change_user_modal_id = "change_user_modal".to_string();

    let open_change_user_modal = open_modal(&change_user_modal_id);

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
            <ChangeUserModal id={change_user_modal_id}/>
        </section>
    }
}

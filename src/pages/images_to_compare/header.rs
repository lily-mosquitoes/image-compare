use yew::{
    classes,
    function_component,
    html,
    use_state_eq,
    Callback,
    Html,
    Properties,
};

use super::change_user_modal::ChangeUserModal;
use crate::{
    pages::markdown_to_yew_html,
    request::User,
    shared_components::Button,
};

static CHANGE_USER_BUTTON: &str =
    include_str!("../../markdown/change_user_button-EN.md");

static FINISH_COMPARING_BUTTON: &str =
    include_str!("../../markdown/finish_comparing_button-EN.md");

#[derive(Properties, PartialEq)]
pub(super) struct HeaderProps {
    pub(super) user: User,
}

#[function_component(Header)]
pub(super) fn header(props: &HeaderProps) -> Html {
    let show_change_user_modal = use_state_eq(|| false);

    let open_change_user_modal = {
        let show_change_user_modal = show_change_user_modal.clone();
        Callback::from(move |_| show_change_user_modal.set(true))
    };

    let close_change_user_modal = {
        let show_change_user_modal = show_change_user_modal.clone();
        Callback::from(move |_| show_change_user_modal.set(false))
    };

    let change_user_button = markdown_to_yew_html(CHANGE_USER_BUTTON);

    let finish_comparing_button = FINISH_COMPARING_BUTTON
        .replace("{votes}", &props.user.votes.to_string());
    let finish_comparing_button =
        markdown_to_yew_html(&finish_comparing_button);

    html! {
        <section
            id="header"
            class={classes![
                "bg-stone-200",
                "drop-shadow-2xl",
                "p-10",
                "flex",
                "flex-row",
                "justify-between",
            ]}
        >
            <Button
                id={"finish_comparing_button"}
                onclick={Callback::from(move |_| ())}
            >
                { finish_comparing_button }
            </Button>
            <Button
                id={"change_user_button"}
                onclick={open_change_user_modal}
            >
                { change_user_button }
            </Button>
            if *show_change_user_modal {
                <ChangeUserModal
                    onclose={close_change_user_modal}
                />
            }
        </section>
    }
}

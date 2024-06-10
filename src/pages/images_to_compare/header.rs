use yew::{
    classes,
    function_component,
    html,
    use_context,
    use_state_eq,
    Callback,
    Html,
    Properties,
    UseReducerHandle,
};

use super::{
    change_user_modal::ChangeUserModal,
    finish_comparing_modal::FinishComparingModal,
};
use crate::{
    pages::markdown_to_yew_html,
    request::User,
    shared_components::Button,
    Language,
};

#[derive(Properties, PartialEq)]
pub(super) struct HeaderProps {
    pub(super) user: User,
    pub(super) onreload: Callback<()>,
}

#[function_component(Header)]
pub(super) fn header(props: &HeaderProps) -> Html {
    let language = match use_context::<UseReducerHandle<Language>>() {
        Some(ctx) => (*ctx).clone(),
        None => Language::default(),
    };
    let show_change_user_modal = use_state_eq(|| false);
    let show_finish_comparing_modal = use_state_eq(|| false);

    let open_change_user_modal = {
        let show_change_user_modal = show_change_user_modal.clone();
        Callback::from(move |_| show_change_user_modal.set(true))
    };

    let close_change_user_modal = {
        let show_change_user_modal = show_change_user_modal.clone();
        Callback::from(move |_| show_change_user_modal.set(false))
    };

    let open_finish_comparing_modal = {
        let show_finish_comparing_modal = show_finish_comparing_modal.clone();
        Callback::from(move |_| show_finish_comparing_modal.set(true))
    };

    let close_finish_comparing_modal = {
        let show_finish_comparing_modal = show_finish_comparing_modal.clone();
        Callback::from(move |_| show_finish_comparing_modal.set(false))
    };

    let change_user_button = language.load_file("change_user_button.md");
    let change_user_button =
        markdown_to_yew_html(change_user_button.unwrap_or(""));

    let finish_comparing_button =
        language.load_file("finish_comparing_button.md");
    let finish_comparing_button = finish_comparing_button
        .unwrap_or("")
        .replace("{votes}", &props.user.votes.to_string());
    let finish_comparing_button =
        markdown_to_yew_html(&finish_comparing_button);

    html! {
        <section
            id="header"
            class={classes![
                "bg-stone-200",
                "drop-shadow-2xl",
                "p-2",
                "flex",
                "flex-row",
                "justify-between",
                "gap-4",
            ]}
        >
            <Button
                id={"finish_comparing_button"}
                onclick={open_finish_comparing_modal}
                class={classes![
                    "border-2",
                    "border-gray-400",
                ]}
            >
                { finish_comparing_button }
            </Button>
            <Button
                id={"change_user_button"}
                onclick={open_change_user_modal}
                class={classes![
                    "border-2",
                    "border-gray-400",
                ]}

            >
                { change_user_button }
            </Button>
            if *show_change_user_modal {
                <ChangeUserModal
                    onclose={close_change_user_modal}
                    onconfirm={props.onreload.clone()}
                />
            }
            if *show_finish_comparing_modal {
                <FinishComparingModal
                    user={props.user.clone()}
                    onclose={close_finish_comparing_modal}
                />
            }
        </section>
    }
}

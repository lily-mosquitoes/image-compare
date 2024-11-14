use yew::{
    classes,
    function_component,
    html,
    Callback,
    Html,
    Properties,
};

#[derive(Clone, PartialEq, Properties)]
pub(super) struct DotButtonProps {
    pub(super) index: u32,
    pub(super) selected: bool,
    pub(super) onclick: Callback<()>,
}

#[function_component(DotButton)]
pub(super) fn dot_button(props: &DotButtonProps) -> Html {
    let bg_classes = match props.selected {
        true => classes!["bg-gray-800"],
        false => classes!["bg-gray-500", "hover:bg-gray-800"],
    };

    let onclick = {
        let event = props.onclick.clone();
        Callback::from(move |_| event.emit(()))
    };

    html! {
        <button
            id={format!("card-{}", props.index)}
            key={format!("card-{}", props.index)}
            class={classes![
                "rounded-full",
                "w-8",
                "h-8",
                "p-3",
                bg_classes,
            ]}
            onclick={onclick}
        />
    }
}

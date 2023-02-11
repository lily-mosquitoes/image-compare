use yew::{
    classes,
    function_component,
    html,
    Html,
    Properties,
};

#[derive(Clone, PartialEq, Properties)]
pub(super) struct DotButtonProps {
    pub(super) index: u32,
    pub(super) selected: bool,
}

#[function_component(DotButton)]
pub(super) fn dot_button(props: &DotButtonProps) -> Html {
    let bg_classes = match props.selected {
        true => classes!["bg-gray-800"],
        false => classes!["bg-gray-500", "hover:bg-gray-800"],
    };

    html! {
        <button
            id={format!("card-{}", props.index)}
            key={format!("card-{}", props.index)}
            class={classes![
                "rounded-full",
                "w-16",
                "h-16",
                "p-3",
                bg_classes,
            ]}
        />
    }
}

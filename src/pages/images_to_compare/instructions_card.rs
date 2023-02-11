use yew::{
    classes,
    function_component,
    html,
    Children,
    Html,
    Properties,
};

#[derive(Clone, PartialEq, Properties)]
pub(super) struct InstructionsCardProps {
    pub(super) id: String,
    pub(super) children: Children,
}

#[function_component(InstructionsCard)]
pub(super) fn instructions_card(
    props: &InstructionsCardProps,
) -> Html {
    html! {
        <div
            id={props.id.clone()}
            class={classes!["text-5xl"]}
        >
            {for props.children.iter()}
        </div>
    }
}

use yew::{
    classes,
    function_component,
    html,
    Callback,
    Html,
    Properties,
};

use crate::{
    request::Image,
    shared_components::Loading,
};

#[derive(Properties, PartialEq)]
pub(super) struct ImageListProps {
    pub(super) loading: bool,
    pub(super) images: Vec<Image>,
    pub(super) onclick: Callback<Image>,
}

#[function_component(ImageList)]
pub(super) fn image_list(props: &ImageListProps) -> Html {
    let onclick = props.onclick.clone();

    props
        .images
        .iter()
        .map(|image| {
            let on_image_select = {
                let onclick = onclick.clone();
                let image = image.clone();
                Callback::from(move |_| onclick.emit(image.clone()))
            };

            if props.loading {
                html! {
                    <button
                        class={classes![
                            "drop-shadow-2xl",
                            "m-10",
                            "p-1",
                            "bg-transparent",
                        ]}
                        disabled=true
                    >
                        <Loading />
                    </button>
                }
            } else {
                html! {
                    <button
                        id={image.id.to_string()}
                        class={classes![
                            "drop-shadow-2xl",
                            "m-10",
                            "p-1",
                            "bg-transparent",
                            "hover:bg-white",
                        ]}
                        onclick={on_image_select}
                    >
                        <div
                            class={classes![
                                "border-4",
                                "border-stone-200",
                                "hover:border-white",
                            ]}
                        >
                            <img
                                class={classes!["w-full", "object-contain"]}
                                src={image.src.clone()}
                            />
                        </div>
                    </button>
                }
            }
        })
        .collect::<Html>()
}

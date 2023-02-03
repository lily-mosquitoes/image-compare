mod change_user_modal;
mod header;
mod image_list;

use yew::{
    function_component,
    html,
    use_effect,
    use_state,
    use_state_eq,
    Callback,
    Html,
};

use self::{
    header::Header,
    image_list::ImageList,
};
use crate::{
    dom::DOM,
    request::{
        get_images,
        Image,
    },
};

#[function_component(ImagesToCompare)]
pub(crate) fn images_to_compare() -> Html {
    let loading = use_state_eq(|| true);
    let image_list = use_state_eq(|| Vec::<Image>::new());
    let selected_image = use_state(|| None);

    {
        let loading = loading.clone();
        let image_list = image_list.clone();

        use_effect(move || {
            if *loading {
                let t = wasm_bindgen::JsValue::from("changing...");
                web_sys::console::log_1(&t);

                wasm_bindgen_futures::spawn_local(async move {
                    let response = get_images().await;
                    image_list.set(response);
                    loading.set(false);
                    let debug_string =
                        wasm_bindgen::JsValue::from("changed");
                    web_sys::console::log_1(&debug_string);
                });
            }
        });
    }

    let on_image_select = {
        let loading = loading.clone();
        let selected_image = selected_image.clone();

        Callback::from(move |image: Image| {
            loading.set(true);
            let debug_string = wasm_bindgen::JsValue::from(format!(
                "image chosen: {},  user agent: {:?}, language: {:?}",
                &image.id.to_string(),
                DOM::user_agent(),
                DOM::language(),
            ));
            web_sys::console::log_1(&debug_string);
            selected_image.set(Some(image));
        })
    };

    html! {
        <section id="compare">
            <Header />
            <section class="flex flex-row">
                <ImageList
                    loading={(*loading).clone()}
                    images={(*image_list).clone()}
                    onclick={on_image_select}
                />
            </section>
        </section>
    }
}

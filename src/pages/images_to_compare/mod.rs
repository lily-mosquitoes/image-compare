mod change_user_modal;
mod header;
mod image_list;

use yew::{
    classes,
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
            <section class={classes!["flex", "flex-row"]}>
                <ImageList
                    loading={(*loading).clone()}
                    images={(*image_list).clone()}
                    onclick={on_image_select}
                />
            </section>
        </section>
    }
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::{
        wasm_bindgen_test,
        wasm_bindgen_test_configure,
    };

    use super::ImagesToCompare;
    use crate::{
        dom::DOM,
        test_helpers::render_yew_component,
    };

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn images_to_compare_has_button_to_change_user() {
        render_yew_component!(ImagesToCompare);

        let buttons = DOM::document()
            .expect("document to be rendered")
            .get_elements_by_tag_name("button");

        let mut index = 0;
        let mut found = false;
        loop {
            match buttons.item(index) {
                Some(button) => {
                    if &button.inner_html() == "Change user" {
                        found = true;
                    }
                },
                None => break,
            }
            index += 1;
        }

        assert!(found);
    }
}

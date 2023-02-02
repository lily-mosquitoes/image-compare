use wasm_bindgen::JsValue;
use yew::prelude::*;

mod change_user_modal;
mod header;
mod image_list;

use header::Header;
use image_list::{
    Image,
    ImageList,
};

#[function_component(ImagesToCompare)]
pub(crate) fn images_to_compare() -> Html {
    let image_list = use_state(|| {
        vec![
            Image {
                id: example_images[i1].0,
                src: example_images[i1].1.to_string(),
            },
            Image {
                id: example_images[i2].0,
                src: example_images[i2].1.to_string(),
            },
        ]
    });

    let selected_image = use_state(|| None);

    let on_image_select = {
        let images = images.clone();
        let selected_image = selected_image.clone();
        let example_images_len = example_images.len();
        let random_index = move || -> usize {
            let mut rng = rand::thread_rng();
            rng.gen_range(0usize..example_images_len)
        };
        Callback::from(move |image: Image| {
            let debug_string = JsValue::from(format!(
                "image chosen: {},  user agent: {}, languages: {:?}",
                &image.id.to_string(),
                web_sys::window()
                    .unwrap()
                    .navigator()
                    .user_agent()
                    .unwrap(),
                web_sys::window().unwrap().navigator().languages()
            ));
            web_sys::console::log_1(&debug_string);
            selected_image.set(Some(image));
            let i1 = random_index();
            let i2 = random_index();
            images.set(vec![
                Image {
                    id: example_images[i1].0,
                    src: example_images[i1].1.to_string(),
                },
                Image {
                    id: example_images[i2].0,
                    src: example_images[i2].1.to_string(),
                },
            ]);
        })
    };

    html! {
        <section id="compare">
            <Header />
            <section class="flex flex-row">
                <ImageList
                    images={(*images).clone()}
                    onclick={on_image_select}
                />
            </section>
        </section>
    }
}

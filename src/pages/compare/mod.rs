use yew::prelude::*;
use wasm_bindgen::JsValue;
use image_compare::wasmjs;
use rand::Rng;

mod header;
mod image;

use header::Header;
use image::{Image, ImageList};

#[function_component(Compare)]
pub(crate) fn compare() -> Html {
    let example_images = vec![
        (0, "static/images/birb-noisy_lambda_=_0.0010000000.png"),
        (1, "static/images/birb-noisy_lambda_=_0.0012593969.png"),
        (2, "static/images/birb-noisy_lambda_=_0.0015860806.png"),
        (3, "static/images/birb-noisy_lambda_=_0.0019975051.png"),
        (4, "static/images/birb-noisy_lambda_=_0.0025156518.png"),
        (5, "static/images/birb-noisy_lambda_=_0.0031682041.png"),
        (6, "static/images/birb-noisy_lambda_=_0.0039900265.png"),
        (7, "static/images/birb-noisy_lambda_=_0.0050250272.png"),
        (8, "static/images/birb-noisy_lambda_=_0.0063285038.png"),
        (9, "static/images/birb-noisy_lambda_=_0.0079700983.png"),
        (10, "static/images/birb-noisy_lambda_=_0.0100375173.png"),
        (11, "static/images/birb-noisy_lambda_=_0.0126412186.png"),
        (12, "static/images/birb-noisy_lambda_=_0.0159203119.png"),
        (13, "static/images/birb-noisy_lambda_=_0.0200499919.png"),
        (14, "static/images/birb-noisy_lambda_=_0.0252508983.png"),
        (15, "static/images/birb-noisy_lambda_=_0.0318009038.png"),
        (16, "static/images/birb-noisy_lambda_=_0.0400499607.png"),
        (17, "static/images/birb-noisy_lambda_=_0.0504387976.png"),
        (18, "static/images/birb-noisy_lambda_=_0.0635224670.png"),
        (19, "static/images/birb-noisy_lambda_=_0.0800000000.png"),
    ];

    let mut rng = rand::thread_rng();
    let mut random_index = || -> usize {
        rng.gen_range(0usize..example_images.len())
    };

    let i1 = random_index();
    let i2 = random_index();
    let images = use_state(|| vec![
        Image {
            id: example_images[i1].0,
            src: example_images[i1].1.to_string(),
        },
        Image {
            id: example_images[i2].0,
            src: example_images[i2].1.to_string(),
        },
    ]);

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
                web_sys::window().unwrap().navigator().user_agent().unwrap(),
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


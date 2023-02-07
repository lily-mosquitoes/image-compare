use serde::{
    Deserialize,
    Serialize,
};

use crate::dom::DOM;

#[derive(Clone, PartialEq, Default, Deserialize, Serialize)]
pub(crate) struct Image {
    pub(crate) id: usize,
    pub(crate) src: String,
}

#[derive(Clone, PartialEq, Default, Deserialize)]
pub(crate) struct ImagesResponse {
    pub(crate) image1: Image,
    pub(crate) image2: Image,
}

impl ImagesResponse {
    pub(crate) fn to_vec(&self) -> Vec<Image> {
        vec![self.image1.clone(), self.image2.clone()]
    }
}

#[cfg(test)]
pub(crate) async fn get_images() -> Result<ImagesResponse, ()> {
    crate::wasm_sleep!(50);

    Ok(ImagesResponse::default())
}

#[cfg(not(test))]
pub(crate) async fn get_images() -> Result<ImagesResponse, ()> {
    yew::platform::time::sleep(std::time::Duration::from_millis(500))
        .await;

    let images_response = ImagesResponse {
        image1: Image {
            id: 0,
            src: "https://i.imgur.com/3ByU8xj.png".to_string(),
        },
        image2: Image {
            id: 1,
            src: "https://i.imgur.com/KN2lyRT.png".to_string(),
        },
    };

    Ok(images_response)
}

#[derive(Serialize)]
pub(crate) struct ChosenImage {
    image: Image,
    user_agent: Option<String>,
    language: Option<String>,
}

impl ChosenImage {
    pub(crate) fn from(image: Image) -> Self {
        ChosenImage {
            image,
            user_agent: DOM::user_agent(),
            language: DOM::language(),
        }
    }
}

pub(crate) async fn post_chosen_image(
    chosen_image: ChosenImage,
) -> Result<(), ()> {
    let debug_string =
        wasm_bindgen::JsValue::from(format!("image was chosen"));
    web_sys::console::log_1(&debug_string);

    Ok(())
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::{
        wasm_bindgen_test,
        wasm_bindgen_test_configure,
    };

    use super::{
        ChosenImage,
        Image,
    };
    use crate::dom::DOM;
    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn chosen_image_struct_contains_user_agent() {
        let test_struct = ChosenImage::from(Image::default());

        assert_eq!(test_struct.user_agent, DOM::user_agent());
    }

    #[wasm_bindgen_test]
    fn chosen_image_struct_contains_language() {
        let test_struct = ChosenImage::from(Image::default());

        assert_eq!(test_struct.language, DOM::language());
    }
}

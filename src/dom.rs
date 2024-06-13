use wasm_bindgen::JsCast;

macro_rules! console_error {
    ($str:expr) => {
        web_sys::console::error_1(&wasm_bindgen::JsValue::from($str))
    };
}
pub(crate) use console_error;

pub(crate) struct DOM;

impl DOM {
    pub(crate) fn window() -> Option<web_sys::Window> {
        web_sys::window()
    }

    pub(crate) fn document() -> Option<web_sys::Document> {
        DOM::window()?.document()
    }

    pub(crate) fn get_element_by_id(id: &str) -> Option<web_sys::Element> {
        DOM::document()?.get_element_by_id(id)
    }

    pub(crate) fn body_first_element_child() -> Option<web_sys::Element> {
        DOM::document()?.body()?.first_element_child()
    }

    pub(crate) fn user_agent() -> Option<String> {
        DOM::window()?.navigator().user_agent().ok()
    }

    pub(crate) fn language() -> Option<String> {
        DOM::window()?.navigator().language()
    }

    pub(crate) fn set_document_language(lang: &str) -> Result<(), &str> {
        DOM::document()
            .ok_or("Document not rendered correctly")?
            .document_element()
            .ok_or("Document Element not redendered correctly")?
            .dyn_into::<web_sys::HtmlElement>()
            .or(Err("Document Element not castable to HtmlElement"))?
            .set_lang(lang);

        Ok(())
    }

    pub(crate) fn local_storage() -> Option<web_sys::Storage> {
        DOM::window()?.local_storage().ok()?
    }
}

#[cfg(test)]
impl DOM {
    pub(crate) fn get_images() -> Option<Vec<web_sys::Element>> {
        let images = DOM::document()?.images();

        let mut images_vec = Vec::<web_sys::Element>::new();
        let mut index = 0;
        loop {
            match images.item(index) {
                Some(image) => {
                    images_vec.push(image);
                },
                None => break,
            }

            index += 1;
        }

        if index > 0 {
            Some(images_vec)
        } else {
            None
        }
    }

    pub(crate) fn get_images_by_id_contains(
        id: &str,
    ) -> Option<Vec<web_sys::Element>> {
        let mut images = DOM::get_images()?;
        images.retain(|x| x.id().contains(id));

        Some(images)
    }

    pub(crate) fn get_button_by_id(id: &str) -> Option<web_sys::Element> {
        let element = DOM::get_element_by_id(id)?;
        match &element.tag_name() == "BUTTON" {
            true => Some(element),
            false => None,
        }
    }

    pub(crate) fn get_buttons_by_id_contains(
        id: &str,
    ) -> Option<Vec<web_sys::Element>> {
        let buttons = DOM::document()?.get_elements_by_tag_name("BUTTON");

        let mut buttons_vec = Vec::<web_sys::Element>::new();
        let mut index = 0;
        loop {
            match buttons.item(index) {
                Some(button) => {
                    if button.id().contains(id) {
                        buttons_vec.push(button);
                    }
                },
                None => break,
            }

            index += 1;
        }

        if index > 0 {
            Some(buttons_vec)
        } else {
            None
        }
    }

    pub(crate) fn has_button_with_inner_html(inner_html: &str) -> bool {
        let buttons = DOM::document()
            .expect("document to be rendered")
            .get_elements_by_tag_name("button");

        let mut index = 0;
        let mut found = false;
        loop {
            match buttons.item(index) {
                Some(button) => {
                    if &button.inner_html() == inner_html {
                        found = true;
                    }
                },
                None => break,
            }
            index += 1;
        }
        found
    }
}

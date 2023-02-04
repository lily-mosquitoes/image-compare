use wasm_bindgen::JsValue;

pub(crate) struct DOM;

impl DOM {
    pub(crate) fn window() -> Option<web_sys::Window> {
        web_sys::window()
    }

    pub(crate) fn document() -> Option<web_sys::Document> {
        DOM::window()?.document()
    }

    pub(crate) fn body_first_element_child(
    ) -> Option<web_sys::Element> {
        DOM::document()?.body()?.first_element_child()
    }

    pub(crate) fn get_element_by_id(
        id: &str,
    ) -> Option<web_sys::Element> {
        DOM::document()?.get_element_by_id(id)
    }

    // uses DomTokenList
    pub(crate) fn add_class_to_element_by_id(
        class: &str,
        id: &str,
    ) -> Result<(), JsValue> {
        if let Some(element) = DOM::get_element_by_id(id) {
            element.class_list().add_1(class)
        } else {
            Err(JsValue::from_str(&format!(
                "could not find element #{}",
                id
            )))
        }
    }

    pub(crate) fn remove_class_from_element_by_id(
        class: &str,
        id: &str,
    ) -> Result<(), JsValue> {
        if let Some(element) = DOM::get_element_by_id(id) {
            element.class_list().remove_1(class)
        } else {
            Err(JsValue::from_str(&format!(
                "could not find element #{}",
                id
            )))
        }
    }

    pub(crate) fn user_agent() -> Option<String> {
        DOM::window()?.navigator().user_agent().ok()
    }

    pub(crate) fn language() -> Option<String> {
        DOM::window()?.navigator().language()
    }
}

#[cfg(test)]
impl DOM {
    pub(crate) fn has_button_with_inner_html(
        inner_html: &str,
    ) -> bool {
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

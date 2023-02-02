use wasm_bindgen::JsValue;

pub(crate) struct DOM;

impl DOM {
    pub(crate) fn window() -> Option<web_sys::Window> {
        web_sys::window()
    }

    pub(crate) fn document() -> Option<web_sys::Document> {
        DOM::window()?.document()
    }

    pub(crate) fn get_element_by_id(
        id: &str,
    ) -> Option<web_sys::Element> {
        DOM::document()?.get_element_by_id(id)
    }

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
}

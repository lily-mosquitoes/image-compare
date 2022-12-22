use wasm_bindgen::JsValue;

pub fn window() -> Option<web_sys::Window> {
    web_sys::window()
}

pub fn document() -> Option<web_sys::Document> {
    window()?.document()
}

pub fn get_element_by_id(id: &str) -> Option<web_sys::Element> {
    document()?.get_element_by_id(id)
}

pub fn add_class_to_element_by_id(class: &str, id: &str) -> Result<(), JsValue> {
    if let Some(element) = get_element_by_id(id) {
        element.class_list().add_1(class)
    } else {
        Err(JsValue::from_str(&format!("could not find element #{}", id)))
    }
}

pub fn remove_class_from_element_by_id(class: &str, id: &str) -> Result<(), JsValue> {
    if let Some(element) = get_element_by_id(id) {
        element.class_list().remove_1(class)
    } else {
        Err(JsValue::from_str(&format!("could not find element #{}", id)))
    }
}


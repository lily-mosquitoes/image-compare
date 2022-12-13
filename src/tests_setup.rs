use wasm_bindgen::JsCast;
use web_sys;

pub static EMPTY_FINGERPRINT: &str = "fingerprint=; \
        expires=Thu, 01 Jan 1970 00:00:00 UTC; \
        path=/;";

#[derive(Debug)]
pub struct WasmWindow;

#[derive(Debug)]
pub struct WasmDocument {
    inner: web_sys::Document,
}

#[derive(Debug)]
pub struct WasmElement {
    inner: web_sys::Element,
}

impl WasmWindow {
    pub fn document() -> WasmDocument {
        let document = web_sys::window()
            .expect("Window to be present")
            .document()
            .expect("Document to be present");

        WasmDocument {
            inner: document,
        }
    }
}

impl WasmDocument {
    pub fn as_html(&self) -> web_sys::HtmlDocument {
        self.inner.to_owned().dyn_into::<web_sys::HtmlDocument>()
            .expect("Document to be castable to HtmlDocument")
    }

    pub fn get_raw_cookies(&self) -> Option<String> {
        self.as_html().cookie().ok()
    }

    pub fn set_raw_cookies(&self, cookie: &str) -> () {
        self.as_html().set_cookie(cookie).expect("cookie to be set")
    }

    pub fn get_element_by_id(&self, id: &str) -> WasmElement {
        let element = self.inner.get_element_by_id(id)
            .expect("Element with id to be present");

        WasmElement {
            inner: element,
        }
    }
}

#[allow(dead_code)]
impl WasmElement {
    pub fn as_html(&self) -> web_sys::HtmlElement {
        self.inner.to_owned().dyn_into::<web_sys::HtmlElement>()
            .expect("Element to be castable to HtmlElement")
    }
}

impl std::ops::Deref for WasmElement {
    type Target = web_sys::Element;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[macro_export]
macro_rules! setup_environment {
    () => {
        use wasm_bindgen_test::*;
        wasm_bindgen_test_configure!(run_in_browser);
    }
}

#[macro_export]
macro_rules! render_app {
    ($component:ident) => {
        yew::Renderer::<$component>::with_root((*WasmWindow::document()
                                               .get_element_by_id("output"))
                                               .to_owned())
            .render();
        // wait for rendering
        yew::platform::time::sleep(std::time::Duration::from_millis(100))
            .await;
    }
}



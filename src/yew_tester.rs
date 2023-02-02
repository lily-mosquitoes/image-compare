macro_rules! render_component {
    ($component:ident) => {
        yew::Renderer::<$component>::with_root(
            (*WasmWindow::document().get_element_by_id("output"))
                .to_owned(),
        )
        .render();

        wait_for_render!();
    };
}
pub(crate) use render_component;

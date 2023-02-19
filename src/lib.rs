pub(crate) mod assets;
pub(crate) mod dom;
pub(crate) mod pages;
pub(crate) mod request;
pub(crate) mod routes;
pub(crate) mod shared_components;

use std::{
    path::PathBuf,
    rc::Rc,
    sync::atomic::{
        AtomicUsize,
        Ordering,
    },
};

use include_dir::{
    include_dir,
    Dir,
};
use lazy_static::lazy_static;
use yew::{
    classes,
    function_component,
    html,
    use_effect,
    use_reducer_eq,
    ContextProvider,
    Html,
    Reducible,
    UseReducerHandle,
};
use yew_router::{
    router::BrowserRouter,
    switch::Switch,
};

use crate::{
    dom::DOM,
    routes::{
        switch,
        Route,
    },
};

static MARKDOWN_DIR: Dir =
    include_dir!("$CARGO_MANIFEST_DIR/src/markdown");

lazy_static! {
    pub(crate) static ref AVAILABLE_LANGUAGES: Vec<PathBuf> =
        MARKDOWN_DIR
            .dirs()
            .map(|d| d.path().to_path_buf())
            .collect();
}

pub(crate) fn load_file_from_language<'a>(
    file: PathBuf,
    lang: usize,
) -> Option<&'static str> {
    match (
        AVAILABLE_LANGUAGES.len() > 0,
        AVAILABLE_LANGUAGES.len() > lang,
    ) {
        (false, _) => None,
        (true, valid) => {
            let index = if valid { lang } else { 0 };
            let mut path = AVAILABLE_LANGUAGES[index].clone();
            path.push(file);
            MARKDOWN_DIR.get_file(&path)?.contents_utf8()
        },
    }
}

pub(crate) static DEFAULT_LANGUAGE: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, PartialEq)]
pub(crate) struct Language {
    pub(crate) index: usize,
}

impl Default for Language {
    fn default() -> Self {
        let index = DEFAULT_LANGUAGE.load(Ordering::SeqCst);
        Self { index }
    }
}

impl Reducible for Language {
    type Action = usize;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        Self { index: action }.into()
    }
}

pub(crate) type LanguageContext = UseReducerHandle<Language>;

#[function_component(App)]
pub fn app() -> Html {
    let language = use_reducer_eq(|| Language::default());

    {
        let language = language.clone();
        use_effect(move || {
            let get_index_of_browser_language =
                || -> Option<usize> {
                    let browser_language =
                        &(DOM::language()?.to_uppercase()[..2]);
                    let browser_language =
                        PathBuf::from(browser_language);
                    let available_lang = AVAILABLE_LANGUAGES
                        .iter()
                        .enumerate()
                        .find(|x| x.1 == &browser_language)?;
                    Some(available_lang.0)
                };
            if let Some(index) = get_index_of_browser_language() {
                language.dispatch(index)
            }
        });
    }

    html! {
        <BrowserRouter>
            <ContextProvider<LanguageContext> context={language}>
                <section
                    id={"main"}
                    class={classes![
                        "h-full",
                        "font-hyperlegible",
                        "bg-gradient-to-tr",
                        "from-stone-700",
                        "via-stone-700",
                        "to-stone-500",
                    ]}
                >
                    <Switch<Route> render={switch} />
                </section>
            </ContextProvider<LanguageContext>>
        </BrowserRouter>
    }
}

#[cfg(test)]
pub(crate) use helpers_for_tests::*;

#[cfg(test)]
mod helpers_for_tests {
    pub(crate) fn markdown_to_decoded_html(text: &str) -> String {
        let html = markdown::to_html(text);
        // insert target="_blank" on links
        let html =
            html.replace("<a href", "<a target=\"_blank\" href");
        // new lines between elements do not get rendered to the DOM
        let html = html.trim().replace(">\n<", "><");
        // encoded characters are escaped when rendered to the DOM
        html_escape::decode_html_entities(&html).into_owned()
    }

    pub(crate) async fn wasm_sleep_in_ms(amount: u64) {
        let duration = std::time::Duration::from_millis(amount);
        yew::platform::time::sleep(duration).await;
    }

    macro_rules! render_yew_component {
        ($component:ident) => {
            yew::Renderer::<$component>::with_root(
                crate::dom::DOM::get_element_by_id("output")
                    .expect("element with id #output to be present"),
            )
            .render();
        };
    }
    pub(crate) use render_yew_component;
}

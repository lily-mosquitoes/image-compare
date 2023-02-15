use std::sync::atomic::Ordering;

use yew::{
    classes,
    function_component,
    html,
    use_context,
    use_state_eq,
    Callback,
    Html,
};

use crate::{
    shared_components::Button,
    Language,
    LanguageContext,
    AVAILABLE_LANGUAGES,
    DEFAULT_LANGUAGE,
};

#[function_component(LanguageButton)]
pub(crate) fn language_button() -> Html {
    let language = use_context::<LanguageContext>();
    let language_menu_visible = use_state_eq(|| false);

    let toggle_language_menu = {
        let language_menu_visible = language_menu_visible.clone();
        Callback::from(move |_| {
            language_menu_visible.set(!*language_menu_visible);
        })
    };

    let change_language = |language_index| {
        let language = language.clone();
        let language_menu_visible = language_menu_visible.clone();
        Callback::from(move |_| {
            match language.clone() {
                Some(ctx) => ctx.dispatch(language_index),
                None => DEFAULT_LANGUAGE
                    .store(language_index, Ordering::SeqCst),
            }
            language_menu_visible.set(false);
        })
    };

    let available_languages: Vec<String> = AVAILABLE_LANGUAGES
        .iter()
        .map(|x| format!("{}", x.display()))
        .collect();

    let language_index = match language.clone() {
        Some(ctx) => ctx.index,
        None => Language::default().index,
    };

    let selected_language = match (
        AVAILABLE_LANGUAGES.len() > 0,
        AVAILABLE_LANGUAGES.len() > language_index,
    ) {
        (true, true) => available_languages[language_index].clone(),
        (true, false) => available_languages[0].clone(),
        (false, _) => "".to_string(),
    };

    html! {
        <>
            if *language_menu_visible {
                <section
                    id={"language_menu"}
                    class={classes![
                        "fixed",
                        "z-10",
                        "bottom-64",
                    ]}
                >
                    <section
                        id={"language_menu_items"}
                        class={classes![
                            "bg-stone-800",
                            "drop-shadow-2xl",
                            "p-12",
                            "rounded-xl",
                            "flex",
                            "flex-col",
                            "gap-8",
                        ]}
                    >
                        {
                            available_languages
                                .iter()
                                .enumerate()
                                .map(|(i, l)| html! {
                                    <Button
                                        id={l.clone()}
                                        key={l.clone()}
                                        class={classes![
                                            "text-5xl",
                                            "text-gray-300",
                                            "aspect-square",
                                        ]}
                                        onclick={change_language(i)}
                                    >
                                        <p>{ l }</p>
                                    </Button>
                                })
                                .collect::<Html>()
                        }
                    </section>
                </section>
            }
            <Button
                id={"select_language_button"}
                class={classes![
                    "text-gray-300",
                    "border-2",
                    "border-gray-300",
                    "w-[2.7em]",
                    "aspect-square",
                ]}
                onclick={toggle_language_menu}
            >
                { selected_language }
            </Button>
        </>
    }
}

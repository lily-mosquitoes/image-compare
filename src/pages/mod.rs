pub(crate) mod images_to_compare;

use yew::{
    AttrValue,
    Html,
};

pub(crate) use self::images_to_compare::ImagesToCompare;

pub(crate) fn markdown_to_yew_html(text: &str) -> Html {
    let html_string = markdown::to_html(text);
    Html::from_html_unchecked(AttrValue::from(html_string))
}

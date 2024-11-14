pub(crate) mod equal_or_different;
pub(crate) mod images_to_compare;

use yew::{
    AttrValue,
    Html,
};

pub(crate) use self::{
    equal_or_different::ImagesToCompare as ExperimentEqualOrDifferent,
    images_to_compare::ImagesToCompare,
};

pub(crate) fn markdown_to_yew_html(text: &str) -> Html {
    let html_string = markdown::to_html(text);
    let html_string =
        html_string.replace("<a href", "<a target=\"_blank\" href");
    Html::from_html_unchecked(AttrValue::from(html_string))
}

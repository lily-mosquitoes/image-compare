#[derive(Clone, PartialEq)]
pub(crate) struct Image {
    pub(crate) id: usize,
    pub(crate) src: String,
}

pub(crate) async fn get_images() -> Vec<Image> {
    yew::platform::time::sleep(std::time::Duration::from_millis(
        2000,
    ))
    .await;

    vec![
        Image {
            id: 0,
            src: "https://i.imgur.com/3ByU8xj.png".to_string(),
        },
        Image {
            id: 1,
            src: "https://i.imgur.com/KN2lyRT.png".to_string(),
        },
    ]
}

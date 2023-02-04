#[derive(Clone, PartialEq)]
pub(crate) struct Image {
    pub(crate) id: usize,
    pub(crate) src: String,
}

#[derive(Clone, PartialEq)]
pub(crate) struct User {
    pub(crate) votes: usize,
}

#[derive(Clone, PartialEq)]
pub(crate) struct Response {
    pub(crate) user: User,
    pub(crate) images: Vec<Image>,
}

pub(crate) async fn get_images() -> Result<Response, ()> {
    yew::platform::time::sleep(std::time::Duration::from_millis(
        1000,
    ))
    .await;

    let user = User { votes: 1 };

    let images = vec![
        Image {
            id: 0,
            src: "https://i.imgur.com/3ByU8xj.png".to_string(),
        },
        Image {
            id: 1,
            src: "https://i.imgur.com/KN2lyRT.png".to_string(),
        },
    ];

    Ok(Response { user, images })
}

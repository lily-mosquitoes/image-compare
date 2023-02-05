use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize)]
pub(crate) struct Image {
    pub(crate) id: usize,
    pub(crate) src: String,
}

#[derive(Deserialize)]
struct ImagesResponse {
    image1: Image,
    image2: Image,
}

impl ImagesResponse {
    fn to_vec(&self) -> Vec<Image> {
        vec![self.image1.clone(), self.image2.clone()]
    }
}

#[cfg(test)]
pub(crate) async fn get_images() -> Result<Vec<Image>, ()> {
    Ok(vec![
        Image {
            id: 0,
            src: "https://i.imgur.com/3ByU8xj.png".to_string(),
        },
        Image {
            id: 1,
            src: "https://i.imgur.com/KN2lyRT.png".to_string(),
        },
    ])
}

#[cfg(not(test))]
pub(crate) async fn get_images() -> Result<Vec<Image>, ()> {
    yew::platform::time::sleep(std::time::Duration::from_millis(500))
        .await;

    let images_response = ImagesResponse {
        image1: Image {
            id: 0,
            src: "https://i.imgur.com/3ByU8xj.png".to_string(),
        },
        image2: Image {
            id: 1,
            src: "https://i.imgur.com/KN2lyRT.png".to_string(),
        },
    };

    Ok(images_response.to_vec())
}

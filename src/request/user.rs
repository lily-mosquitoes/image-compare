use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize, Default)]
pub(crate) struct User {
    pub(crate) votes: usize,
    pub(crate) average_chosen_lambda: Option<f64>,
}

#[cfg(test)]
pub(crate) async fn get_user() -> Result<User, ()> {
    Ok(User::default())
}

#[cfg(not(test))]
pub(crate) async fn get_user() -> Result<User, ()> {
    yew::platform::time::sleep(std::time::Duration::from_millis(500))
        .await;

    let user = User::default();

    Ok(user)
}

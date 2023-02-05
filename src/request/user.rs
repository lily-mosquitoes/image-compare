use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize, Default)]
pub(crate) struct User {
    pub(crate) votes: usize,
    pub(crate) average_chosen_lambda: Option<f64>,
}

#[cfg(test)]
lazy_static::lazy_static! {
    static ref VOTES_FOR_TESTING: bool = rand::random();
}

#[cfg(test)]
pub(crate) async fn get_user() -> Result<User, ()> {
    let mut user = User::default();
    if *VOTES_FOR_TESTING {
        user.votes = 1;
    }

    Ok(user)
}

#[cfg(not(test))]
pub(crate) async fn get_user() -> Result<User, ()> {
    yew::platform::time::sleep(std::time::Duration::from_millis(500))
        .await;

    let mut user = User::default();
    user.votes = 1;

    Ok(user)
}

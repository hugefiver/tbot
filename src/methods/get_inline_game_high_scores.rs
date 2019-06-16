use super::*;
use crate::{internal::Client, types::game::HighScore};

/// Represents the [`getGameHighScores`][docs] method for inline messages.
///
/// [docs]: https://core.telegram.org/bots/api#getgamehighscores
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct GetInlineGameHighScores<'a, C> {
    #[serde(skip)]
    client: &'a Client<C>,
    #[serde(skip)]
    token: Token,
    user_id: i64,
    inline_message_id: &'a str,
}

impl<'a, C> GetInlineGameHighScores<'a, C> {
    pub(crate) const fn new(
        client: &'a Client<C>,
        token: Token,
        inline_message_id: &'a str,
        user_id: i64,
    ) -> Self {
        Self {
            client,
            token,
            user_id,
            inline_message_id,
        }
    }
}

impl<C> IntoFuture for GetInlineGameHighScores<'_, C>
where
    C: hyper::client::connect::Connect + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
    type Future =
        Box<dyn Future<Item = Self::Item, Error = Self::Error> + Send>;
    type Item = Vec<HighScore>;
    type Error = DeliveryError;

    fn into_future(self) -> Self::Future {
        Box::new(send_method(
            self.client,
            &self.token,
            "getGameHighScores",
            None,
            serde_json::to_vec(&self).unwrap(),
        ))
    }
}

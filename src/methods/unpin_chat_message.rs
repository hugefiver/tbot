use super::*;
use crate::{
    connectors::Connector,
    errors,
    internal::{BoxFuture, Client},
    types::parameters::{ChatId, ImplicitChatId},
};

/// Unpins a chat message.
///
/// Reflects the [`unpinChatMessage`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#unpinchatmessage
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct UnpinChatMessage<'a, C> {
    #[serde(skip)]
    client: &'a Client<C>,
    #[serde(skip)]
    token: Token,
    chat_id: ChatId<'a>,
}

impl<'a, C> UnpinChatMessage<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: Token,
        chat_id: impl ImplicitChatId<'a>,
    ) -> Self {
        Self {
            client,
            token,
            chat_id: chat_id.into(),
        }
    }
}

impl<C: Connector> IntoFuture for UnpinChatMessage<'_, C> {
    type Future = BoxFuture<Self::Item, Self::Error>;
    type Item = ();
    type Error = errors::MethodCall;

    fn into_future(self) -> Self::Future {
        Box::new(
            send_method::<bool, C>(
                self.client,
                &self.token,
                "unpinChatMessage",
                None,
                serde_json::to_vec(&self).unwrap(),
            )
            .map(|_| ()), // Only `true` is returned on success
        )
    }
}

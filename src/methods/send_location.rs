use super::*;

/// Representation of the [`SendLocation`] method.
///
/// [`SendLocation`]: https://core.telegram.org/bots/api#sendlocation
#[derive(Serialize)]
pub struct SendLocation<'a> {
    #[serde(skip)]
    token: &'a str,
    chat_id: types::ChatId<'a>,
    latitude: f64,
    longitude: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    live_period: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<types::raw::Keyboard<'a>>,
}

impl<'a> SendLocation<'a> {
    /// Constructs a new `SendLocation`.
    #[must_use]
    pub fn new<'b: 'a>(
        token: &'b str,
        chat_id: impl Into<types::ChatId<'b>>,
        latitude: f64,
        longitude: f64,
    ) -> SendLocation<'a> {
        SendLocation {
            token,
            chat_id: chat_id.into(),
            latitude,
            longitute,
            live_period: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    /// Sets `live_period` to `Some(duration)`.
    #[must_use]
    pub fn live_period(mut self, duration: u16) -> Self {
        self.live_period = Some(duration);
        self
    }

    /// Sets `disable_notification` to `Some(is_disabled)`.
    #[must_use]
    pub fn disable_notification(mut self, is_disabled: bool) -> Self {
        self.disable_notification = Some(is_disabled);
        self
    }

    /// Sets `reply_to_message_id` to `Some(id)`.
    #[must_use]
    pub fn reply_to_message_id(mut self, id: u64) -> Self {
        self.reply_to_message_id = Some(id);
        self
    }

    /// Sets `reply_markup` to `Some(markup)`.
    #[must_use]
    pub fn reply_markup(
        mut self,
        markup: impl Into<types::raw::Keyboard<'a>>,
    ) -> Self {
        self.reply_markup = Some(markup.into());
        self
    }

    /// Prepares the request and returns a `Future`.
    #[must_use]
    pub fn into_future(
        self,
    ) -> impl Future<Item = types::raw::Message, Error = DeliveryError> {
        send_method::<types::raw::Message>(
            self.token,
            "sendLocation",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
    }
}
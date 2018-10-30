use super::*;

/// Representation of the [`getMe`] method.
///
/// [`getMe`]: https://core.telegram.org/bots/api#getme
pub struct GetMe<'a> {
    token: &'a str,
}

impl<'a> GetMe<'a> {
    /// Creates a new `GetMe`.
    #[must_use]
    pub fn new<'b: 'a>(token: &'b str) -> GetMe {
        GetMe {
            token,
        }
    }

    /// Prepares the request and returns a `Future`.
    #[must_use]
    pub fn get_request(
        &self,
    ) -> impl Future<Item = types::User, Error = DeliveryError> {
        send_method::<types::User>(self.token, "getMe", None, Vec::new())
    }
}

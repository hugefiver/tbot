use super::*;

/// Represents a [`Contact`].
///
/// [`Contact`]: https://core.telegram.org/bots/api#contact
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
pub struct Contact {
    /// The phone number of the contact.
    pub phone_number: String,
    /// The first name of the contact.
    pub first_name: String,
    /// The last name of the contact.
    pub last_name: Option<String>,
    /// The user id of the contact.
    pub user_id: Option<i64>,
    /// The vCard of the contact.
    pub vcard: Option<String>,
}

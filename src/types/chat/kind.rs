use crate::types::Message;

/// Represents the kind of a chat.
#[derive(Debug, PartialEq, Clone)]
// todo: #[non_exhaustive]
pub enum Kind {
    /// The chat is private.
    // todo: #[non_exhaustive]
    Private {
        /// The username of the user.
        username: Option<String>,
        /// The first name of the user.
        first_name: String,
        /// The last name of the user.
        last_name: Option<String>,
    },
    /// The chat is a group.
    // todo: #[non_exhaustive]
    Group {
        /// The title of the group.
        title: String,
        /// `true` if all membmers of this group have admin rights.
        all_members_are_administrators: bool,
        /// The pinned message of the group.
        pinned_message: Option<Box<Message>>,
    },
    /// The chat is a supergroup.
    // todo: #[non_exhaustive]
    Supergroup {
        /// The title of the supergroup.
        title: String,
        /// The username of the supergroup.
        username: Option<String>,
        /// The description of the supergroup.
        description: Option<String>,
        /// The invite link of the supergroup.
        invite_link: Option<String>,
        /// The pinned message of the supergroup.
        pinned_message: Option<Box<Message>>,
        /// The name of the sticker set of the supergroup.
        sticker_set_name: Option<String>,
        /// `true` if the bot can set the sticker set of the supergroup.
        can_set_sticker_set: Option<bool>,
    },
    /// The chat is a channel.
    // todo: #[non_exhaustive]
    Channel {
        /// The title of the channel.
        title: String,
        /// The username of the channel.
        username: Option<String>,
        /// The description of the channel.
        description: Option<String>,
        /// The invite link of the channel.
        invite_link: Option<String>,
        /// The pinned message of the channel.
        pinned_message: Option<Box<Message>>,
    },
}

use super::MockBot;
use crate::{methods::*, types, types::input_file::*};

/// Provides methods that infer data from the message and simplify calling
/// certain chat methods.
pub trait ChatMethods<'a> {
    #[doc(hidden)]
    fn bot(&self) -> &MockBot;
    #[doc(hidden)]
    fn chat_id(&self) -> i64;
    #[doc(hidden)]
    fn message_id(&self) -> u32;

    /// Deletes the photo of this chat.
    fn delete_chat_photo(&'a self) -> DeleteChatPhoto<'a> {
        self.bot().delete_chat_photo(self.chat_id())
    }

    /// Deletes the sticker set of this chat.
    fn delete_chat_sticker_set(&'a self) -> DeleteChatStickerSet<'a> {
        self.bot().delete_chat_sticker_set(self.chat_id())
    }

    /// Deletes a message in this chat.
    fn delete_message(&'a self, message_id: u32) -> DeleteMessage<'a> {
        self.bot().delete_message(self.chat_id(), message_id)
    }

    /// Deletes the incoming message.
    fn delete_this_message(&'a self) -> DeleteMessage<'a> {
        self.delete_message(self.message_id())
    }

    /// Updates the caption of a message in this group.
    fn edit_message_caption(
        &'a self,
        message_id: u32,
        caption: &'a str,
    ) -> EditMessageCaption<'a> {
        self.bot().edit_message_caption(self.chat_id(), message_id, caption)
    }

    /// Updates a live location in this group.
    fn edit_message_location(
        &'a self,
        message_id: u32,
        location: (f64, f64),
    ) -> EditMessageLocation<'a> {
        self.bot().edit_message_location(self.chat_id(), message_id, location)
    }

    /// Updates the media of a message in this group. 
    fn edit_message_media(
        &'a self,
        message_id: u32,
        media: impl Into<EditableMedia<'a>>,
    ) -> EditMessageMedia<'a> {
        self.bot().edit_message_media(self.chat_id(), message_id, media)
    }

    /// Updates the reply markup of a message in this group. 
    fn edit_message_reply_markup(
        &'a self,
        message_id: u32,
        reply_markup: types::InlineKeyboard<'a>,
    ) -> EditMessageReplyMarkup<'a> {
        self.bot().edit_message_reply_markup(
            self.chat_id(),
            message_id,
            reply_markup,
        )
    }

    /// Updates the text of a message in this group.
    fn edit_message_text(
        &'a self,
        message_id: u32,
        text: &'a str,
    ) -> EditMessageText<'a> {
        self.bot().edit_message_text(self.chat_id(), message_id, text)
    }

    /// Exports the invite link of this chat.
    fn export_chat_invite_link(&'a self) -> ExportChatInviteLink<'a> {
        self.bot().export_chat_invite_link(self.chat_id())
    }

    /// Forwards a message to this chat.
    fn forward_here(
        &'a self,
        from_chat_id: impl Into<types::ChatId<'a>>,
        message_id: u32,
    ) -> ForwardMessage<'a> {
        self.bot().forward_message(self.chat_id(), from_chat_id, message_id)
    }

    /// Gets information about this chat.
    fn get_chat(&'a self) -> GetChat<'a> {
        self.bot().get_chat(self.chat_id())
    }

    /// Gets a list of admins of this chat.
    fn get_chat_administrators(&'a self) -> GetChatAdministrators<'a> {
        self.bot().get_chat_administrators(self.chat_id())
    }

    /// Gets information about a member of this chat.
    fn get_chat_member(&'a self, user_id: i64) -> GetChatMember<'a> {
        self.bot().get_chat_member(self.chat_id(), user_id)
    }

    /// Gets the number of members in this chat.
    fn get_chat_members_count(&'a self) -> GetChatMembersCount<'a> {
        self.bot().get_chat_members_count(self.chat_id())
    }

    /// Gets infomation about high scores in a game sent in this chat.
    fn get_message_game_high_scores(
        &'a self,
        message_id: u32,
        user_id: i64,
    ) -> GetMessageGameHighScores<'a> {
        self.bot().get_message_game_high_scores(
            self.chat_id(),
            message_id,
            user_id,
        )
    }

    /// Kicks a member of this chat.
    fn kick_chat_member(&'a self, user_id: i64) -> KickChatMember<'a> {
        self.bot().kick_chat_member(self.chat_id(), user_id)
    }

    /// Leaves this chat.
    fn leave_chat(&'a self) -> LeaveChat<'a> {
        self.bot().leave_chat(self.chat_id())
    }

    /// Promotes a member of this chat.
    fn promote_chat_member(&'a self, user_id: i64) -> PromoteChatMember<'a> {
        self.bot().promote_chat_member(self.chat_id(), user_id)
    }

    /// Restricts a member of this chat.
    fn restrict_chat_member(&'a self, user_id: i64) -> RestrictChatMember<'a> {
        self.bot().restrict_chat_member(self.chat_id(), user_id)
    }

    /// Send an animation to this chat.
    fn send_animation(
        &'a self,
        animation: &'a Animation<'a>,
    ) -> SendAnimation<'a> {
        self.bot().send_animation(self.chat_id(), animation)
    }

    /// Sends an animation in reply to this message.
    fn send_animation_in_reply(
        &'a self,
        animation: &'a Animation<'a>,
    ) -> SendAnimation<'a> {
        self.send_animation(animation).reply_to_message_id(self.message_id())
    }

    /// Sends an audio to this chat.
    fn send_audio(&'a self, audio: &'a Audio<'a>) -> SendAudio<'a> {
        self.bot().send_audio(self.chat_id(), audio)
    }

    /// Sends an audio in reply to this message.
    fn send_audio_in_reply(&'a self, audio: &'a Audio<'a>) -> SendAudio<'a> {
        self.send_audio(audio).reply_to_message_id(self.message_id())
    }

    /// Sends an action to this group.
    fn send_chat_action(&self, action: types::ChatAction) -> SendChatAction {
        self.bot().send_chat_action(self.chat_id(), action)
    }

    /// Sends a contact to this group.
    fn send_contact(
        &'a self,
        phone_number: &'a str,
        first_name: &'a str,
    ) -> SendContact<'a> {
        self.bot().send_contact(self.chat_id(), phone_number, first_name)
    }

    /// Sends a contact in reply to this message.
    fn send_contact_in_reply(
        &'a self,
        phone_number: &'a str,
        first_name: &'a str,
    ) -> SendContact<'a> {
        self.send_contact(phone_number, first_name)
            .reply_to_message_id(self.message_id())
    }

    /// Sends a game to this chat.
    fn send_game(&'a self, game_short_name: &'a str) -> SendGame<'a> {
        self.bot().send_game(self.chat_id(), game_short_name)
    }

    /// Sends a game in reply to this message.
    fn send_game_in_reply(&'a self, game_short_name: &'a str) -> SendGame<'a> {
        self.send_game(game_short_name).reply_to_message_id(self.message_id())
    }

    /// Sends a document to this chat.
    fn send_document(&'a self, document: &'a Document<'a>) -> SendDocument<'a> {
        self.bot().send_document(self.chat_id(), document)
    }

    /// Sends a document in reply to this message.
    fn send_document_in_reply(
        &'a self,
        document: &'a Document<'a>,
    ) -> SendDocument<'a> {
        self.send_document(document).reply_to_message_id(self.message_id())
    }

    /// Sends a location to this chat.
    fn send_location(&self, location: (f64, f64)) -> SendLocation<'_> {
        self.bot().send_location(self.chat_id(), location)
    }

    /// Sends a location in reply to this message.
    fn send_location_in_reply(&self, location: (f64, f64)) -> SendLocation<'_> {
        self.send_location(location).reply_to_message_id(self.message_id())
    }

    /// Sends an album to this chat.
    fn send_media_group(
        &'a self,
        media: Vec<GroupMedia<'a>>,
    ) -> SendMediaGroup<'a> {
        self.bot().send_media_group(self.chat_id(), media)
    }

    /// Sends an album in reply to this message.
    fn send_media_group_in_reply(
        &'a self,
        media: Vec<GroupMedia<'a>>,
    ) -> SendMediaGroup<'a> {
        self.send_media_group(media).reply_to_message_id(self.message_id())
    }

    /// Sends a message to this chat.
    fn send_message(&'a self, text: &'a str) -> SendMessage<'a> {
        self.bot().send_message(self.chat_id(), text)
    }

    /// Sends a message in reply to this message.
    fn send_message_in_reply(&'a self, text: &'a str) -> SendMessage<'a> {
        self.send_message(text).reply_to_message_id(self.message_id())
    }

    /// Sends a photo to this chat.
    fn send_photo(&'a self, photo: &'a Photo<'a>) -> SendPhoto<'a> {
        self.bot().send_photo(self.chat_id(), photo)
    }

    /// Sends a photo in reply to this message.
    fn send_photo_in_reply(&'a self, photo: &'a Photo<'a>) -> SendPhoto<'a> {
        self.send_photo(photo).reply_to_message_id(self.message_id())
    }

    /// Sends a poll to this chat.
    fn send_poll(
        &'a self,
        question: &'a str,
        options: &'a [&'a str],
    ) -> SendPoll<'a> {
        self.bot().send_poll(self.chat_id(), question, options)
    }

    /// Sends a poll in reply to this message.
    fn send_poll_in_reply(
        &'a self,
        question: &'a str,
        options: &'a [&'a str],
    ) -> SendPoll<'a> {
        self.send_poll(question, options).reply_to_message_id(self.message_id())
    }

    /// Sends a sticker to this chat.
    fn send_sticker(&'a self, sticker: &'a Sticker<'a>) -> SendSticker<'a> {
        self.bot().send_sticker(self.chat_id(), sticker)
    }

    /// Sends a sticker in reply to this message.
    fn send_sticker_in_reply(
        &'a self,
        sticker: &'a Sticker<'a>,
    ) -> SendSticker<'a> {
        self.send_sticker(sticker).reply_to_message_id(self.message_id())
    }

    /// Sends a venue to this chat.
    fn send_venue(
        &'a self,
        location: (f64, f64),
        title: &'a str,
        address: &'a str,
    ) -> SendVenue<'a> {
        self.bot().send_venue(self.chat_id(), location, title, address)
    }

    /// Sends a venue in reply to this message.
    fn send_venue_in_reply(
        &'a self,
        location: (f64, f64),
        title: &'a str,
        address: &'a str,
    ) -> SendVenue<'a> {
        self.send_venue(location, title, address)
            .reply_to_message_id(self.message_id())
    }

    /// Sends a video to this chat.
    fn send_video(&'a self, video: &'a Video<'a>) -> SendVideo<'a> {
        self.bot().send_video(self.chat_id(), video)
    }

    /// Sends a video in reply to this message.
    fn send_video_in_reply(&'a self, video: &'a Video<'a>) -> SendVideo<'a> {
        self.send_video(video).reply_to_message_id(self.message_id())
    }

    /// Sends a video note to this chat.
    fn send_video_note(
        &'a self,
        video_note: &'a VideoNote<'a>,
    ) -> SendVideoNote<'a> {
        self.bot().send_video_note(self.chat_id(), video_note)
    }

    /// Sends a video note in reply to this message.
    fn send_video_note_in_reply(
        &'a self,
        video_note: &'a VideoNote<'a>,
    ) -> SendVideoNote<'a> {
        self.send_video_note(video_note).reply_to_message_id(self.message_id())
    }

    /// Sends a voice to this chat.
    fn send_voice(&'a self, voice: &'a Voice<'a>) -> SendVoice<'a> {
        self.bot().send_voice(self.chat_id(), voice)
    }

    /// Sends a voice in reply to this message.
    fn send_voice_in_reply(&'a self, voice: &'a Voice<'a>) -> SendVoice<'a> {
        self.send_voice(voice).reply_to_message_id(self.message_id())
    }

    /// Sets a new description of this chat.
    fn set_chat_description(
        &'a self,
        description: &'a str,
    ) -> SetChatDescription<'a> {
        self.bot().set_chat_description(self.chat_id(), description)
    }

    /// Sets a new photo of this chat.
    fn set_chat_photo(&'a self, photo: &'a ChatPhoto<'a>) -> SetChatPhoto<'a> {
        self.bot().set_chat_photo(self.chat_id(), photo)
    }

    /// Sets a new sticker set of this chat.
    fn set_chat_sticker_set(
        &'a self,
        sticker_set_name: &'a str,
    ) -> SetChatStickerSet<'a> {
        self.bot().set_chat_sticker_set(self.chat_id(), sticker_set_name)
    }

    /// Sets a new chat title of this chat.
    fn set_chat_title(&'a self, title: &'a str) -> SetChatTitle<'a> {
        self.bot().set_chat_title(self.chat_id(), title)
    }

    /// Sets a new high score for a player who played a game in this chat.
    fn set_message_game_score(
        &'a self,
        message_id: u32,
        user_id: i64,
        score: u32,
    ) -> SetMessageGameScore<'a> {
        self.bot().set_message_game_score(
            self.chat_id(),
            message_id,
            user_id,
            score,
        )
    }

    /// Unbans a member of this chat.
    fn unban_chat_member(&'a self, user_id: i64) -> UnbanChatMember<'a> {
        self.bot().unban_chat_member(self.chat_id(), user_id)
    }

    /// Unpins the pinned message in this chat.
    fn unpin_chat_message(&'a self) -> UnpinChatMessage<'a> {
        self.bot().unpin_chat_message(self.chat_id())
    }
}

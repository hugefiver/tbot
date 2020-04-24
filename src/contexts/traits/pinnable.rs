use super::ChatMethods;
use crate::methods::PinChatMessage;

/// Provides methods for pinnable messages.
pub trait Pinnable<C: 'static>: ChatMethods<C> {
    /// Pins this message.
    fn pin_this_message(&self) -> PinChatMessage<'_, C> {
        self.bot()
            .pin_chat_message(self.chat().id, self.message_id())
    }
}

use super::{InputFile, WithName};
use crate::types::parameters::{ParseMode, Text};
use serde::Serialize;
use std::borrow::Cow;

/// Represents a voice to be sent.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
#[must_use]
pub struct Voice<'a> {
    pub(crate) media: WithName<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) duration: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) caption: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) parse_mode: Option<ParseMode>,
}

impl<'a> Voice<'a> {
    fn new(media: InputFile<'a>) -> Self {
        Self {
            media: media.with_name("voice"),
            duration: None,
            caption: None,
            parse_mode: None,
        }
    }

    /// Constructs a `Voice` from bytes.
    pub fn bytes(bytes: impl Into<Cow<'a, [u8]>>) -> Self {
        Self::new(InputFile::File {
            filename: "voice.ogg".into(),
            bytes: bytes.into(),
        })
    }

    /// Constructs a `Voice` from a file ID.
    ///
    /// # Panics
    ///
    /// Panicks if the ID starts with `attach://`.
    pub fn id(id: impl Into<Cow<'a, str>>) -> Self {
        let id = id.into();
        assert!(
            !id.starts_with("attach://"),
            "\n[tbot]: Voice's ID cannot start with `attach://`\n",
        );

        Self::new(InputFile::Id(id))
    }

    /// Constructs a `Voice` from an URL.
    ///
    /// # Panics
    ///
    /// Panicks if the URL starts with `attach://`.
    pub fn url(url: impl Into<Cow<'a, str>>) -> Self {
        let url = url.into();
        assert!(
            !url.starts_with("attach://"),
            "\n[tbot]: Voice's URL cannot start with `attach://`\n",
        );

        Self::new(InputFile::Url(url))
    }
    /// Configures `duration`.
    pub fn duration(mut self, duration: u32) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Configures `caption`.
    pub fn caption(mut self, caption: impl Into<Text<'a>>) -> Self {
        let caption = caption.into();

        self.caption = Some(caption.text.into());
        self.parse_mode = caption.parse_mode;
        self
    }
}

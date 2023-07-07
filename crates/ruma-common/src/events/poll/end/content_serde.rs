use serde::{Deserialize, Serialize};

use crate::events::{
    message::{historical_serde::MessageContentBlock, TextContentBlock},
    relation::Reference,
};

use super::{PollEndEventContent, PollResultsContentBlock};

#[derive(Deserialize)]
#[serde(untagged)]
pub(super) enum PollEndEventContentDeHelper {
    Unstable(PollEndEventContentUnstableSerDeHelper),
    Stable(PollEndEventContentStableDeHelper),
}

impl From<PollEndEventContentDeHelper> for PollEndEventContent {
    fn from(value: PollEndEventContentDeHelper) -> Self {
        match value {
            PollEndEventContentDeHelper::Unstable(h) => h.into(),
            PollEndEventContentDeHelper::Stable(h) => h.into(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub(super) struct PollEndEventContentUnstableSerDeHelper {
    #[serde(flatten)]
    message: MessageContentBlock,

    #[serde(rename = "org.matrix.msc3381.poll.end")]
    poll_end: PollEndContentBlockUnstableSerDeHelper,

    #[cfg(feature = "unstable-msc3955")]
    #[serde(
        default,
        skip_serializing_if = "crate::serde::is_default",
        rename = "org.matrix.msc1767.automated"
    )]
    automated: bool,

    #[serde(rename = "m.relates_to")]
    relates_to: Reference,
}

impl From<PollEndEventContentUnstableSerDeHelper> for PollEndEventContent {
    fn from(value: PollEndEventContentUnstableSerDeHelper) -> Self {
        let PollEndEventContentUnstableSerDeHelper {
            message,
            #[cfg(feature = "unstable-msc3955")]
            automated,
            relates_to,
            ..
        } = value;

        Self {
            text: message.into(),
            poll_results: None,
            #[cfg(feature = "unstable-msc3955")]
            automated,
            relates_to,
        }
    }
}

impl From<PollEndEventContent> for PollEndEventContentUnstableSerDeHelper {
    fn from(value: PollEndEventContent) -> Self {
        let PollEndEventContent {
            text,
            #[cfg(feature = "unstable-msc3955")]
            automated,
            relates_to,
            ..
        } = value;

        Self {
            message: text.into(),
            poll_end: PollEndContentBlockUnstableSerDeHelper {},
            #[cfg(feature = "unstable-msc3955")]
            automated,
            relates_to,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct PollEndContentBlockUnstableSerDeHelper {}

#[derive(Deserialize)]
pub(super) struct PollEndEventContentStableDeHelper {
    #[serde(rename = "m.text")]
    text: TextContentBlock,

    #[serde(rename = "m.poll.results", skip_serializing_if = "Option::is_none")]
    pub poll_results: Option<PollResultsContentBlock>,

    #[cfg(feature = "unstable-msc3955")]
    #[serde(default, rename = "org.matrix.msc1767.automated")]
    pub automated: bool,

    #[serde(rename = "m.relates_to")]
    pub relates_to: Reference,
}

impl From<PollEndEventContentStableDeHelper> for PollEndEventContent {
    fn from(value: PollEndEventContentStableDeHelper) -> Self {
        let PollEndEventContentStableDeHelper {
            text,
            poll_results,
            #[cfg(feature = "unstable-msc3955")]
            automated,
            relates_to,
        } = value;

        Self {
            text,
            poll_results,
            #[cfg(feature = "unstable-msc3955")]
            automated,
            relates_to,
        }
    }
}

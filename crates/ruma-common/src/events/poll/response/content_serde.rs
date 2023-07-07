use serde::{Deserialize, Serialize};

use crate::events::relation::Reference;

use super::{PollResponseEventContent, SelectionsContentBlock};

#[derive(Deserialize)]
#[serde(untagged)]
pub(super) enum PollResponseEventContentDeHelper {
    Unstable(PollResponseEventContentUnstableSerDeHelper),
    Stable(PollResponseEventContentStableDeHelper),
}

impl From<PollResponseEventContentDeHelper> for PollResponseEventContent {
    fn from(value: PollResponseEventContentDeHelper) -> Self {
        match value {
            PollResponseEventContentDeHelper::Unstable(h) => h.into(),
            PollResponseEventContentDeHelper::Stable(h) => h.into(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub(super) struct PollResponseEventContentUnstableSerDeHelper {
    #[serde(rename = "org.matrix.msc3381.poll.response")]
    response: ResponseContentBlockUnstableSerDeHelper,

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

impl From<PollResponseEventContentUnstableSerDeHelper> for PollResponseEventContent {
    fn from(value: PollResponseEventContentUnstableSerDeHelper) -> Self {
        let PollResponseEventContentUnstableSerDeHelper {
            response,
            #[cfg(feature = "unstable-msc3955")]
            automated,
            relates_to,
        } = value;

        Self {
            selections: response.into(),
            #[cfg(feature = "unstable-msc3955")]
            automated,
            relates_to,
        }
    }
}

impl From<PollResponseEventContent> for PollResponseEventContentUnstableSerDeHelper {
    fn from(value: PollResponseEventContent) -> Self {
        let PollResponseEventContent {
            selections,
            #[cfg(feature = "unstable-msc3955")]
            automated,
            relates_to,
        } = value;

        Self {
            response: selections.into(),
            #[cfg(feature = "unstable-msc3955")]
            automated,
            relates_to,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
struct ResponseContentBlockUnstableSerDeHelper {
    answers: Vec<String>,
}

impl From<ResponseContentBlockUnstableSerDeHelper> for SelectionsContentBlock {
    fn from(value: ResponseContentBlockUnstableSerDeHelper) -> Self {
        Self(value.answers)
    }
}

impl From<SelectionsContentBlock> for ResponseContentBlockUnstableSerDeHelper {
    fn from(value: SelectionsContentBlock) -> Self {
        Self { answers: value.0 }
    }
}

#[derive(Deserialize)]
pub(super) struct PollResponseEventContentStableDeHelper {
    #[serde(rename = "m.selections")]
    selections: SelectionsContentBlock,

    #[cfg(feature = "unstable-msc3955")]
    #[serde(default, rename = "org.matrix.msc1767.automated")]
    automated: bool,

    #[serde(rename = "m.relates_to")]
    relates_to: Reference,
}

impl From<PollResponseEventContentStableDeHelper> for PollResponseEventContent {
    fn from(value: PollResponseEventContentStableDeHelper) -> Self {
        let PollResponseEventContentStableDeHelper {
            selections,
            #[cfg(feature = "unstable-msc3955")]
            automated,
            relates_to,
        } = value;

        Self {
            selections,
            #[cfg(feature = "unstable-msc3955")]
            automated,
            relates_to,
        }
    }
}

use js_int::UInt;
use serde::{de, Deserialize, Serialize};

use crate::events::message::{historical_serde::MessageContentBlock, TextContentBlock};

use super::{
    PollAnswer, PollAnswers, PollContentBlock, PollKind, PollQuestion, PollStartEventContent,
};

#[derive(Deserialize)]
#[serde(untagged)]
pub(super) enum PollStartEventContentDeHelper {
    Unstable(PollStartEventContentUnstableSerDeHelper),
    Stable(PollStartEventContentStableDeHelper),
}

impl From<PollStartEventContentDeHelper> for PollStartEventContent {
    fn from(value: PollStartEventContentDeHelper) -> Self {
        match value {
            PollStartEventContentDeHelper::Unstable(h) => h.into(),
            PollStartEventContentDeHelper::Stable(h) => h.into(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub(super) struct PollStartEventContentUnstableSerDeHelper {
    #[serde(rename = "org.matrix.msc3381.poll.start")]
    poll_start: PollStartContentBlockUnstableSerDeHelper,

    #[serde(flatten)]
    message: MessageContentBlock,

    #[cfg(feature = "unstable-msc3955")]
    #[serde(
        default,
        skip_serializing_if = "crate::serde::is_default",
        rename = "org.matrix.msc1767.automated"
    )]
    automated: bool,
}

impl From<PollStartEventContentUnstableSerDeHelper> for PollStartEventContent {
    fn from(value: PollStartEventContentUnstableSerDeHelper) -> Self {
        let PollStartEventContentUnstableSerDeHelper {
            poll_start,
            message,
            #[cfg(feature = "unstable-msc3955")]
            automated,
        } = value;

        Self {
            poll: poll_start.into(),
            text: message.into(),
            #[cfg(feature = "unstable-msc3955")]
            automated,
        }
    }
}

impl From<PollStartEventContent> for PollStartEventContentUnstableSerDeHelper {
    fn from(value: PollStartEventContent) -> Self {
        let PollStartEventContent {
            poll,
            text,
            #[cfg(feature = "unstable-msc3955")]
            automated,
        } = value;

        Self {
            poll_start: poll.into(),
            message: text.into(),
            #[cfg(feature = "unstable-msc3955")]
            automated,
        }
    }
}

/// A block for poll content.
#[derive(Serialize, Deserialize)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
struct PollStartContentBlockUnstableSerDeHelper {
    question: PollQuestionUnstableSerDeHelper,

    #[serde(default)]
    kind: PollKind,

    #[serde(
        default = "PollContentBlock::default_max_selections",
        skip_serializing_if = "PollContentBlock::max_selections_is_default"
    )]
    max_selections: UInt,

    answers: PollAnswersUnstableSerDeHelper,
}

impl From<PollStartContentBlockUnstableSerDeHelper> for PollContentBlock {
    fn from(value: PollStartContentBlockUnstableSerDeHelper) -> Self {
        let PollStartContentBlockUnstableSerDeHelper { question, kind, max_selections, answers } =
            value;

        Self { question: question.into(), kind, max_selections, answers: answers.into() }
    }
}

impl From<PollContentBlock> for PollStartContentBlockUnstableSerDeHelper {
    fn from(value: PollContentBlock) -> Self {
        let PollContentBlock { question, kind, max_selections, answers } = value;

        Self { question: question.into(), kind, max_selections, answers: answers.into() }
    }
}

#[derive(Serialize, Deserialize)]
struct PollQuestionUnstableSerDeHelper {
    #[serde(flatten)]
    message: MessageContentBlock,
}

impl From<PollQuestionUnstableSerDeHelper> for PollQuestion {
    fn from(value: PollQuestionUnstableSerDeHelper) -> Self {
        Self { text: value.message.into() }
    }
}

impl From<PollQuestion> for PollQuestionUnstableSerDeHelper {
    fn from(value: PollQuestion) -> Self {
        Self { message: value.text.into() }
    }
}

#[derive(Serialize)]
struct PollAnswersUnstableSerDeHelper(Vec<PollAnswerUnstableSerDeHelper>);

impl<'de> Deserialize<'de> for PollAnswersUnstableSerDeHelper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut answers = Vec::<PollAnswerUnstableSerDeHelper>::deserialize(deserializer)?;

        if answers.is_empty() {
            return Err(de::Error::invalid_length(0, &"at least 1 answer"));
        }

        answers.truncate(20);
        Ok(Self(answers))
    }
}

impl From<PollAnswersUnstableSerDeHelper> for PollAnswers {
    fn from(value: PollAnswersUnstableSerDeHelper) -> Self {
        Self(value.0.into_iter().map(Into::into).collect())
    }
}

impl From<PollAnswers> for PollAnswersUnstableSerDeHelper {
    fn from(value: PollAnswers) -> Self {
        Self(value.0.into_iter().map(Into::into).collect())
    }
}

#[derive(Serialize, Deserialize)]
struct PollAnswerUnstableSerDeHelper {
    id: String,

    /// The text representation of the answer.
    #[serde(flatten)]
    message: MessageContentBlock,
}

impl From<PollAnswerUnstableSerDeHelper> for PollAnswer {
    fn from(value: PollAnswerUnstableSerDeHelper) -> Self {
        let PollAnswerUnstableSerDeHelper { id, message } = value;

        Self { id, text: message.into() }
    }
}

impl From<PollAnswer> for PollAnswerUnstableSerDeHelper {
    fn from(value: PollAnswer) -> Self {
        let PollAnswer { id, text } = value;

        Self { id, message: text.into() }
    }
}

#[derive(Deserialize)]
pub(super) struct PollStartEventContentStableDeHelper {
    #[serde(rename = "m.poll")]
    pub poll: PollContentBlock,

    #[serde(rename = "m.text")]
    pub text: TextContentBlock,

    #[cfg(feature = "unstable-msc3955")]
    #[serde(default, rename = "org.matrix.msc1767.automated")]
    pub automated: bool,
}

impl From<PollStartEventContentStableDeHelper> for PollStartEventContent {
    fn from(value: PollStartEventContentStableDeHelper) -> Self {
        let PollStartEventContentStableDeHelper {
            poll,
            text,
            #[cfg(feature = "unstable-msc3955")]
            automated,
        } = value;

        Self {
            poll,
            text,
            #[cfg(feature = "unstable-msc3955")]
            automated,
        }
    }
}

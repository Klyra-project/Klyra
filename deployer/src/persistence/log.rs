use chrono::{DateTime, Utc};
use serde_json::Value;
use klyra_common::log::BuildLogStream;
use klyra_common::STATE_MESSAGE;
use uuid::Uuid;

use super::{deploy_layer::extract_message, State};

#[derive(Clone, Debug, Eq, PartialEq, sqlx::FromRow)]
pub struct Log {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub state: State,
    pub level: Level,
    pub file: Option<String>,
    pub line: Option<u32>,
    pub target: String,
    pub fields: serde_json::Value,
}

#[derive(Clone, Debug, Eq, PartialEq, sqlx::Type)]
pub enum Level {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl Log {
    pub fn into_build_log_stream(self) -> Option<BuildLogStream> {
        // Only state transition logs is a Value::String
        let (state, message) = if let Value::String(str_value) = &self.fields {
            if str_value == STATE_MESSAGE {
                match self.state {
                    State::Queued => Some((klyra_common::deployment::State::Queued, None)),
                    State::Building => Some((klyra_common::deployment::State::Building, None)),
                    State::Built => Some((klyra_common::deployment::State::Built, None)),
                    State::Running => Some((klyra_common::deployment::State::Running, None)),
                    State::Completed => Some((klyra_common::deployment::State::Completed, None)),
                    State::Stopped => Some((klyra_common::deployment::State::Stopped, None)),
                    State::Crashed => Some((klyra_common::deployment::State::Crashed, None)),
                    State::Unknown => Some((klyra_common::deployment::State::Unknown, None)),
                }
            } else {
                None
            }
        } else {
            match self.state {
                State::Building => {
                    let msg = extract_message(&self.fields)?;
                    Some((klyra_common::deployment::State::Building, Some(msg)))
                }
                _ => None,
            }
        }?;

        Some(BuildLogStream {
            id: self.id,
            timestamp: self.timestamp,
            state,
            message,
        })
    }
}

impl From<Log> for klyra_common::LogItem {
    fn from(log: Log) -> Self {
        Self {
            id: log.id,
            state: log.state.into(),
            timestamp: log.timestamp,
            level: log.level.into(),
            file: log.file,
            line: log.line,
            target: log.target,
            fields: serde_json::to_vec(&log.fields).unwrap(),
        }
    }
}

impl From<Level> for klyra_common::log::Level {
    fn from(level: Level) -> Self {
        match level {
            Level::Trace => Self::Trace,
            Level::Debug => Self::Debug,
            Level::Info => Self::Info,
            Level::Warn => Self::Warn,
            Level::Error => Self::Error,
        }
    }
}

impl From<klyra_common::log::Level> for Level {
    fn from(level: klyra_common::log::Level) -> Self {
        match level {
            klyra_common::log::Level::Trace => Self::Trace,
            klyra_common::log::Level::Debug => Self::Debug,
            klyra_common::log::Level::Info => Self::Info,
            klyra_common::log::Level::Warn => Self::Warn,
            klyra_common::log::Level::Error => Self::Error,
        }
    }
}

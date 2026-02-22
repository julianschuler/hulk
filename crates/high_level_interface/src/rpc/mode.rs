use serde::{Deserialize, Serialize};

use crate::error::Error;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(into = "usize", try_from = "usize")]
pub enum Mode {
    Damping = 0,
    Prepare = 1,
    Walking = 2,
    Custom = 3,
    Soccer = 4,
}

impl From<Mode> for usize {
    fn from(mode: Mode) -> Self {
        mode as usize
    }
}

impl TryFrom<usize> for Mode {
    type Error = Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Mode::Damping),
            1 => Ok(Mode::Prepare),
            2 => Ok(Mode::Walking),
            3 => Ok(Mode::Custom),
            4 => Ok(Mode::Soccer),
            _ => Err(Error::UnknownMode(value)),
        }
    }
}

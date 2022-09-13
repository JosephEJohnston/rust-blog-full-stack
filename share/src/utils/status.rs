use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct StatusOptions {
    pub is_all: bool,
    pub status: Option<i8>,
}


use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TmuxSessionState {
    pub is_new_session: bool,
}

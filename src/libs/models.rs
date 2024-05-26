use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: u64,
    pub name: String,
    pub completed: bool,
}

use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Resource)]
pub struct CheckSeqence(pub Vec<usize>);

impl Default for CheckSeqence {
    fn default() -> Self {
        CheckSeqence(vec![])
    }
}

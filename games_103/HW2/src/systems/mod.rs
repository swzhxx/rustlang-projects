use bevy::prelude::*;

use crate::components::{Mass, ELV};

pub fn collision_handling() {}

// 隐式积分更新
pub fn implict_model(
    mut commands: Commands,
    mesh: Query<&Handle<Mesh>>,
    cloth: Query<(Entity, &Mass, &mut ELV)>,
) {
}

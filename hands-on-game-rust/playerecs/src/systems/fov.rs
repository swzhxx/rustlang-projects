use legion::world::SubWorld;

use crate::prelude::*;

#[system]
#[read_component(Point)]
#[write_component(FieldOfView)]
pub fn fov(ecs: &mut SubWorld, #[resource] map: &Map) {
    let mut views = <(&Point, &mut FieldOfView)>::query();
    views.iter_mut(ecs).for_each(|(pos, mut fov)| {
        fov.visible_tiles = field_of_view_set(*pos, fov.radius, map);
        fov.is_dirty = false;
    })
}

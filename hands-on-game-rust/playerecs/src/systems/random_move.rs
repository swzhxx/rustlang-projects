use crate::prelude::*;
use legion::{systems::CommandBuffer, world::SubWorld};
#[system]
#[write_component(Point)]
#[read_component(MovingRandomly)]
pub fn random_move(ecs: &mut SubWorld, commands: &mut CommandBuffer, #[resource] map: &Map) {
    let mut movers = <(Entity, &mut Point, &MovingRandomly)>::query();
    movers.iter_mut(ecs).for_each(|(entity, pos, _)| {
        let mut rng = RandomNumberGenerator::new();
        let destination = match rng.range(0, 4) {
            0 => Point::new(-1, 0),
            1 => Point::new(1, 0),
            2 => Point::new(0, -1),
            _ => Point::new(0, 1),
        } + *pos;
        commands.push((
            (),
            WantsToMove {
                entity: *entity,
                destination,
            },
        ));
    });
}

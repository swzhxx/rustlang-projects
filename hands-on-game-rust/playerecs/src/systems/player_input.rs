use legion::{systems::CommandBuffer, world::SubWorld};

use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
#[read_component(Health)]
pub fn player_input(
    ecs: &mut SubWorld,
    #[resource] map: &Map,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,

    #[resource] turn_state: &mut TurnState,
) {
    let mut players = <(Entity, &Point)>::query().filter(component::<Player>());

    //START: delta
    if let Some(key) = *key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::new(0, 0),
        };
        let (player_entity, destination) = players
            .iter(ecs)
            .find_map(|(entity, pos)| Some((*entity, *pos + delta)))
            .unwrap();
        let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());
        let mut did_somthing = false;
        if delta.x != 0 || delta.y != 0 {
            let mut hit_something = false;

            enemies
                .iter(ecs)
                .filter(|(_, pos)| **pos == destination)
                .for_each(|(entity, _)| {
                    hit_something = true;
                    did_somthing = true;
                    println!("attack");
                    commands.push((
                        (),
                        WantsToAttack {
                            attacker: player_entity,
                            target: *entity,
                        },
                    ));
                });
            if !hit_something {
                did_somthing = true;
                commands.push((
                    (),
                    WantsToMove {
                        entity: player_entity,
                        destination,
                    },
                ));
            }
        }
        if !did_somthing {
            if let Ok(mut health) = ecs
                .entry_mut(player_entity)
                .unwrap()
                .get_component_mut::<Health>()
            {
                health.current = i32::min(health.max, health.current + 1)
            }
        }
        *turn_state = TurnState::PlayerTurn;
    }
}

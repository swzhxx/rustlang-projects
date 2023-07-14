use bevy::prelude::*;
use bevy_mod_picking::PickingEvent;

use crate::{components::CheckNode, resources::CheckSeqence};

pub fn pick_events(
    mut events: EventReader<PickingEvent>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut check_sequence: ResMut<CheckSeqence>,
    mut query: Query<(
        Entity,
        &mut CheckNode,
        &mut Handle<StandardMaterial>,
        &Interaction,
    )>,
) {
    for event in events.iter() {
        match event {
            PickingEvent::Selection(e) => info!("A selection event happened: {:?}", e),
            PickingEvent::Hover(e) => {
                // info!("hover {:?}", e);
                match e {
                    bevy_mod_picking::HoverEvent::JustEntered(entity) => {
                        if let Ok((entity, mut check_node, mut material_handler, _)) =
                            query.get_mut(entity.clone())
                        {
                            if let Some(material) = materials.get_mut(&material_handler) {
                                material.base_color = Color::rgb(0., 1., 0.);
                            }
                        }
                    }
                    bevy_mod_picking::HoverEvent::JustLeft(entity) => {
                        if let Ok((_, mut check_node, mut material_handler, _)) =
                            query.get_mut(entity.clone())
                        {
                            if let Some(material) = materials.get_mut(&material_handler) {
                                material.base_color = if check_node.checked == false {
                                    Color::rgb(1., 1., 1.)
                                } else {
                                    Color::rgb(1., 0., 0.)
                                }
                            }
                        }
                    }
                }
            }
            PickingEvent::Clicked(e) => {
                info!("clicked {:?}", e);
                if let Ok((entity, mut check_node, mut material_handler, _)) =
                    query.get_mut(e.clone())
                {
                    if check_node.checked == true {
                        check_node.checked = false;
                        check_sequence.as_mut().0.retain(|x| *x != check_node.index);
                    } else {
                        check_node.checked = true;
                        check_sequence.as_mut().0.push(check_node.index)
                    }
                    if let Some(material) = materials.get_mut(&material_handler) {
                        material.base_color = if check_node.checked == false {
                            // info!("修改白色");
                            Color::rgb(1., 1., 1.)
                        } else {
                            // info!("修改红色");
                            Color::rgb(1., 0., 0.)
                        }
                    }
                }
            }
        }
    }
}

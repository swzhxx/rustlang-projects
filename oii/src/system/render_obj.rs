use crate::{
    components::{ModifyPickedFile, PickedFiles, VerticeNodes},
    utils::create_mesh,
};
use bevy::pbr::wireframe::Wireframe;
use bevy::prelude::*;
use nalgebra::{point, Isometry};
use obj::raw::object::Polygon;
use rapier3d::parry::bounding_volume;
use std::{fs::File, io::BufReader};

pub fn render_obj(
    modify_picked_file_query: Query<(Entity, &ModifyPickedFile)>,
    mut query: Query<&mut PickedFiles>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query_mesh: Query<(Entity, &Handle<Mesh>)>,
) {
    let mut picked_files = query.single_mut();
    for (modify_entity, modify_event) in modify_picked_file_query.iter() {
        if let Some(current_index) = modify_event.current_index {
            let file = &picked_files.files[current_index];
            if let Ok(res) = File::open(&file.path) {
                let input = BufReader::new(res);
                if let Ok(model) = obj::raw::parse_obj(input) {
                    let deltas = Isometry::identity();
                    let mut vertices: Vec<_> = model
                        .positions
                        .iter()
                        .map(|v| point![v.0, v.1, v.2])
                        .collect();
                    let aabb = bounding_volume::details::point_cloud_aabb(&deltas, &vertices);
                    let center = aabb.center();
                    let diag = (aabb.maxs - aabb.mins).norm();
                    vertices
                        .iter_mut()
                        .for_each(|p| *p = (*p - center.coords) * 6.0 / diag);
                    let vertices = vertices.iter().fold(vec![], |mut acc, v| {
                        acc.push([v[0], v[1], v[2]]);
                        acc
                    });
                    let indices: Vec<_> = model
                        .polygons
                        .into_iter()
                        .flat_map(|p| match p {
                            Polygon::P(idx) => idx.into_iter(),
                            Polygon::PT(idx) => {
                                Vec::from_iter(idx.into_iter().map(|i| i.0)).into_iter()
                            }
                            Polygon::PN(idx) => {
                                Vec::from_iter(idx.into_iter().map(|i| i.0)).into_iter()
                            }
                            Polygon::PTN(idx) => {
                                Vec::from_iter(idx.into_iter().map(|i| i.0)).into_iter()
                            }
                        })
                        .collect();
                    let indices: Vec<_> = indices
                        .chunks(3)
                        .map(|idx| [idx[0] as u32, idx[1] as u32, idx[2] as u32])
                        .collect();
                    let indices = indices.iter().fold(vec![], |mut acc, v| {
                        acc.push(v[0]);
                        acc.push(v[1]);
                        acc.push(v[2]);
                        acc
                    });
                    let mesh = create_mesh(&indices, &vertices);
                    let handle = meshes.add(mesh);

                    if picked_files.current_entity.is_some() {
                        commands
                            .entity(picked_files.current_entity.unwrap())
                            .despawn_recursive();
                    }
                    let entity = commands
                        .spawn(PbrBundle {
                            mesh: handle.clone(),
                            material: materials.add(StandardMaterial {
                                base_color: Color::rgb(1., 1., 1.),
                                ..default()
                            }),
                            ..default()
                        })
                        .id();
                    picked_files.current_entity = Some(entity);
                    VerticeNodes::create_with_entity(
                        &mut commands,
                        entity,
                        &handle,
                        &query_mesh,
                        &mut meshes,
                        &mut materials,
                    );
                    picked_files.current_index = Some(current_index);
                }
            }
        }
        commands.entity(modify_entity).despawn_recursive();
    }
}

use bevy::{prelude::*, render::mesh::Indices};
use bevy_rapier3d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_physics)
        .add_system(print_ball_altitude)
        .add_system(display_events)
        .add_system(display_intersection_info)
        .run();
}

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-3.0, 3.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

fn setup_physics(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(100.0, 0.1, 100.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)));

    let shape = shape::Cube::default();
    let mesh = Mesh::from(shape);
    let mesh_handle = meshes.add(mesh.clone());
    /* Create the bouncing ball. */
    // commands
    //     .spawn(RigidBody::Dynamic)
    //     .insert(Sensor)
    //     .insert(Collider::ball(0.5))
    //     .insert(Restitution::coefficient(0.7))
    //     .insert(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)));

    commands
        .spawn(PbrBundle {
            mesh: mesh_handle.clone(),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.3, 0.5, 0.3),
                metallic: 0.2,
                perceptual_roughness: 0.5,
                ..default()
            }),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Sensor)
        .insert(Collider::from_bevy_mesh(&mesh, &ComputedColliderShape::TriMesh).unwrap())
        .insert(Restitution::coefficient(0.7));
}

fn print_ball_altitude(positions: Query<&Transform, With<RigidBody>>) {
    for transform in positions.iter() {
        // println!("Ball altitude: {}", transform.translation.y);
    }
}

fn display_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
) {
    for collision_event in collision_events.iter() {
        println!("Received collision event: {:?}", collision_event);
    }

    for contact_force_event in contact_force_events.iter() {
        println!("Received contact force event: {:?}", contact_force_event);
    }
}

fn display_intersection_info(
    rapier_context: Res<RapierContext>,
    query: Query<(Entity, &Handle<Mesh>, &Transform, With<RigidBody>)>,
    meshes: Res<Assets<Mesh>>,
) {
    for (entity, handle_mesh, transform, _) in query.iter() {
        /* Iterate through all the intersection pairs involving a specific collider. */
        for (collider1, collider2, intersecting) in rapier_context.intersections_with(entity) {
            println!("intersecting {:?}", intersecting);
            if intersecting == false {
                break;
            }
            let other_collider = if entity == collider1 {
                collider2
            } else {
                collider1
            };
            if let Some(mesh) = meshes.get(handle_mesh) {
                let vertices_ref = get_mesh_vertices(mesh);
                let vertices = vertices_ref
                    .iter()
                    .map(|vertice| {
                        return transform.transform_point(vertice.clone());
                    })
                    .collect::<Vec<Vec3>>();
                let edges = get_mesh_edge(mesh);

                let mut collider_vertices: Vec<Vec3> = vec![];
                let mut collider_edges: Vec<Vec3> = vec![];

                for vertice in vertices.iter() {
                    rapier_context.intersections_with_point(
                        vertice.clone(),
                        QueryFilter::default(),
                        |entity| {
                            println!("callback {:?} {:?}", entity, vertice);
                            true
                            // if entity != other_collider {
                            //     false
                            // } else {
                            //     true
                            // }
                        },
                    )
                }
                // rapier_context.intersection_pair(other_collider, collider2);
            }
        }
    }
}

fn get_mesh_vertices(mesh: &Mesh) -> Vec<Vec3> {
    let vertices = mesh
        .attribute(Mesh::ATTRIBUTE_POSITION)
        .unwrap()
        .as_float3()
        .unwrap();

    let res = vertices
        .iter()
        .map(move |v| {
            let _v = Vec3::new(v[0], v[1], v[2]);
            _v
        })
        .collect();

    return res;
}

fn get_mesh_edge(mesh: &Mesh) -> Vec<(usize, usize)> {
    let indices = mesh.indices().unwrap();
    let mut edges = vec![];
    let mut temp = vec![];
    for index in indices.iter() {
        if temp.len() < 3 {
            temp.push(index);
        } else {
            edges.push((temp[0], temp[1]));
            edges.push((temp[1], temp[2]));
            edges.push((temp[2], temp[0]));
            temp.clear();
        }
    }
    edges
}

fn get_mesh_edge_len(vertices: &Vec<Vec3>, edges: &Vec<(usize, usize)>) -> Vec<f32> {
    todo!()
}

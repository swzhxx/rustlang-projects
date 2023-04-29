use bevy::{
    ecs::{entity, system::EntityCommands, world::World},
    prelude::*,
};
use bevy_mod_picking::PickableBundle;

#[derive(Debug)]
pub struct FileDescriptor {
    pub path: String,
    pub file_name: String,
}

#[derive(Component, Default)]
pub struct PickedFiles {
    pub picked_folder_path: Option<String>,
    pub files: Vec<FileDescriptor>,
    pub current_index: Option<usize>,
    pub current_entity: Option<Entity>,
    pub show_selected_point: bool,
}

#[derive(Component, Default)]
pub struct ModifyPickedFile {
    pub old_index: Option<usize>,
    pub current_index: Option<usize>,
}

#[derive(Component)]
pub struct VerticeNodes {
    entity: Option<Entity>,
}

impl VerticeNodes {
    pub fn new() -> Self {
        let nodes = VerticeNodes { entity: None };
        nodes
    }
    pub fn create_with_entity(
        commands: &mut Commands,
        root_entity: Entity,
        handle_mesh: &Handle<Mesh>,
        query: &Query<(Entity, &Handle<Mesh>)>,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> anyhow::Result<()> {
        let mut vertice_nodes = VerticeNodes::new();
        let handle = meshes.add(
            Mesh::try_from(shape::Icosphere {
                radius: 0.03,
                ..default()
            })
            .unwrap(),
        );
        let mesh = meshes.get(handle_mesh);
        if mesh.is_none() {
            error!("mesh not found")
        }
        let entity = commands.spawn_empty().id();
        vertice_nodes.entity = Some(entity);
        commands
            .entity(entity)
            .insert(vertice_nodes)
            .insert(PbrBundle { ..default() });
        let vertices = mesh
            .unwrap()
            .attribute(Mesh::ATTRIBUTE_POSITION)
            .unwrap()
            .as_float3()
            .unwrap();
        for (index, vertice) in vertices.iter().enumerate() {
            let check_node = CheckNode {
                index,
                checked: false,
            };
            let node_entity = Self::add(commands, &handle, check_node, vertice, materials);
            commands.get_entity(entity).unwrap().add_child(node_entity);
        }
        commands.get_entity(root_entity).unwrap().add_child(entity);
        Ok(())
    }

    pub fn get_child_check_nodes<'a>(&'a self, world: &'a mut World) -> Vec<(Entity, &CheckNode)> {
        let mut query = world.query::<(Entity, &Children)>();
        let mut query_child = world.query::<(Entity, &CheckNode)>();
        let mut v = vec![];
        let (entity, children) = query.get(world, self.entity.unwrap()).unwrap();
        for child in children.iter() {
            if let Ok((_, checkNode)) = query_child.get(world, child.clone()) {
                v.push((child.clone(), checkNode))
            }
        }
        v
    }

    fn add(
        commands: &mut Commands,
        mesh_handle: &Handle<Mesh>,
        check_node: CheckNode,
        position: &[f32; 3],
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> Entity {
        let e = commands
            .spawn((
                PbrBundle {
                    mesh: mesh_handle.clone(),
                    material: materials.add(StandardMaterial::from(Color::rgb(1., 1., 1.))),
                    transform: Transform::from_translation(Vec3::new(
                        position[0],
                        position[1],
                        position[2],
                    )),
                    ..default()
                },
                PickableBundle::default(),
            ))
            .insert(check_node)
            .id();
        e
    }
}

#[derive(Component)]
pub struct CheckNode {
    pub index: usize,
    pub checked: bool,
}

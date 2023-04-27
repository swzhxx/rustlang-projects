use bevy::{
    ecs::{system::EntityCommands, world},
    prelude::*,
};

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
pub struct VerticeNodes;

impl VerticeNodes {
    pub fn new() -> Self {
        let nodes = VerticeNodes;
        nodes
    }
    pub fn create_with_entity(
        mut commands: Commands,
        root_entity: Entity,
        handle_mesh: &Handle<Mesh>,
        query: Query<(Entity, &Handle<Mesh>)>,
        mut meshes: ResMut<Assets<Mesh>>,
    ) -> anyhow::Result<()> {
        let mut vertice_nodes = VerticeNodes::new();
        let handle = meshes.add(
            Mesh::try_from(shape::Icosphere {
                radius: 0.3,
                ..default()
            })
            .unwrap(),
        );
        let mesh = meshes.get(handle_mesh);
        if mesh.is_none() {
            error!("mesh not found")
        }
        let entity = commands.spawn(vertice_nodes).id();
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
            let node_entity = vertice_nodes.add(&mut commands, &handle, check_node, vertice);
            commands.get_entity(entity).unwrap().add_child(node_entity);
        }

        commands.get_entity(root_entity).unwrap().add_child(entity);

        Ok(())
    }

    pub fn get_child_check_nodes(&self, mut world: World) -> Vec<(Entity, &CheckNode)> {
        let query = world.query::<(Entity, &CheckNode)>();
        // for query.iter()
        todo!()
    }

    fn add(
        &mut self,
        commands: &mut Commands,
        mesh_handle: &Handle<Mesh>,
        check_node: CheckNode,
        position: &[f32; 3],
    ) -> Entity {
        let e = commands
            .spawn(PbrBundle {
                mesh: mesh_handle.clone(),
                transform: Transform::from_translation(Vec3::new(
                    position[0],
                    position[1],
                    position[2],
                )),
                visibility: match check_node.checked {
                    false => Visibility::Hidden,
                    true => Visibility::Inherited,
                },
                ..default()
            })
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

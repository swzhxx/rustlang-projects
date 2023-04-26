use bevy::{ecs::system::EntityCommands, prelude::*};

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
    nodes: Vec<Entity>,
}

impl VerticeNodes {
    pub fn new() -> Self {
        let nodes = VerticeNodes { nodes: vec![] };
        nodes
    }
    pub fn create_with_entity(
        mut commands: Commands,
        query: Query<(Entity, &Handle<Mesh>)>,
        mut meshes: ResMut<Assets<Mesh>>,
    ) -> anyhow::Result<Self> {
        let (entity, handle_mesh) = query.single();
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
            let node_entity =
                vertice_nodes.add(entity, &mut commands, &handle, check_node, vertice);
            vertice_nodes.nodes.push(node_entity);
        }
        Ok(vertice_nodes)
    }
    fn add(
        &mut self,
        parent_entity: Entity,
        commands: &mut Commands,
        mesh_handle: &Handle<Mesh>,
        check_node: CheckNode,
        position: &[f32; 3],
    ) -> Entity {
        let e = commands
            .get_entity(parent_entity)
            .unwrap()
            .insert(PbrBundle {
                mesh: mesh_handle.clone(),
                transform: Transform::from_translation(Vec3::new(
                    position[0],
                    position[1],
                    position[2],
                )),
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

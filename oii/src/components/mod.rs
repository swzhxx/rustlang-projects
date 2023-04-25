use bevy::prelude::*;

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
    pub nodes: Vec<CheckNode>,
    pub entity: Option<Entity>,
}

impl VerticeNodes {
    pub fn new(mut commands: Commands, mut query: Query<&mut VerticeNodes>) -> Entity {
        let mut nodes = VerticeNodes {
            nodes: vec![],
            entity: None,
        };
        let entity = commands.spawn(nodes).id();
        // nodes.entity = Some(entity.clone());
        query
            .get_component_mut::<VerticeNodes>(entity)
            .unwrap()
            .entity = Some(entity);
        entity
    }
    pub fn add(&mut self, mut commands: Commands) -> Entity {
        // commands.entity(self.entity).spawn()
        todo!()
    }
}

#[derive(Component)]
pub struct CheckNode {
    pub root_entity: Entity,
    pub index: usize,
    pub checked: bool,
    pub ball_entity: Entity,
}

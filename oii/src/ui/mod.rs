use bevy::{
    pbr::wireframe::WireframeConfig,
    prelude::{Query, *},
    tasks::IoTaskPool,
};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use rfd;
use std::{
    fs::{self, File},
    io::Write,
};

use crate::components::{CheckNode, FileDescriptor, ModifyPickedFile, PickedFiles, VerticeNodes};

pub fn ui_system<'a>(
    mut contexts: EguiContexts,
    mut picked_files_query: Query<&mut PickedFiles>,
    mut commands: Commands,
    mut wireframe_config: ResMut<WireframeConfig>,
    nodes_query: Query<(Entity, &VerticeNodes)>,
    child_query: Query<(Entity, &Children)>,
    checknode_query: Query<(Entity, &'a CheckNode)>,
) {
    egui::Window::new("file").show(contexts.ctx_mut(), |ui| {
        let mut picked_files = picked_files_query.single_mut();
        if ui.button("Open file...").clicked() {
            if let Some(path) = rfd::FileDialog::new().pick_folder() {
                picked_files.picked_folder_path = Some(path.display().to_string());
                info!("{:?}", picked_files.picked_folder_path);
                if let Some(folder_path) = &picked_files.picked_folder_path {
                    for entry in fs::read_dir(folder_path).unwrap() {
                        let entry = entry.unwrap();

                        let path = entry.path();
                        if get_suffix_name(&path.display().to_string()).eq(&Some("obj".to_string()))
                        {
                            // picked_files.files.push(path.display().to_string());
                            picked_files.files.push(FileDescriptor {
                                path: path.display().to_string(),
                                file_name: entry.file_name().to_str().unwrap().to_string(),
                            });
                        }
                        info!("picked files {:?}", picked_files.files);
                    }
                }
                if picked_files.files.len() >= 1 {
                    // picked_files.current_index = Some(0)
                    commands.spawn(ModifyPickedFile {
                        old_index: picked_files.current_index.clone(),
                        current_index: Some(0),
                    });
                }
            }
        }
        ui.separator();
        // ui.checkbox(, text)
        ui.checkbox(&mut wireframe_config.global, "wiframe");
        ui.separator();
        ui.checkbox(
            &mut picked_files.show_selected_point,
            "display selectable vertices ball",
        );
        ui.separator();
        if ui.button("Save").clicked() {
            if picked_files.picked_folder_path.is_some() && picked_files.current_index.is_some() {
                let (node_entity, _) = nodes_query
                    .get(picked_files.current_entity.as_ref().unwrap().clone())
                    .unwrap();
                let (_, children) = child_query.get(node_entity).unwrap();
                let mut v = vec![];
                for child in children.iter() {
                    if let Ok((_, checkNode)) = checknode_query.get(child.clone()) {
                        v.push(checkNode)
                    }
                }

                let path = picked_files.files[picked_files.current_index.unwrap()]
                    .path
                    .to_string();
                let json = serde_json::to_string(&v).unwrap();
                IoTaskPool::get().spawn(async move {
                    File::create(format!("{}.json", &path))
                        .and_then(|mut file| file.write(json.as_bytes()))
                        .expect("Error Save Failed");
                });
            }
        }
        ui.separator();
        ui.collapsing("Click to see what is hidden!", |ui| {
            // ui.label("Not much, as it turns out");
            for (index, file) in picked_files.files.iter().enumerate() {
                let mut label = file.file_name.to_string();
                if Some(index) == picked_files.current_index {
                    label = "> ".to_string() + &file.file_name.to_string();
                }
                if ui.button(&label).clicked() {
                    info!("click index {:?} file name {:?}", index, file.file_name);
                    commands.spawn(ModifyPickedFile {
                        old_index: picked_files.current_index.clone(),
                        current_index: Some(index),
                    });
                }
            }
        });
    });
}

fn get_suffix_name(str: &str) -> Option<String> {
    let temp: Vec<&str> = str.split('.').collect();
    if temp.len() == 1 {
        None
    } else {
        Some((*(temp.last().unwrap())).to_owned())
    }
}
#[cfg(test)]
mod test {
    use super::get_suffix_name;

    #[test]
    fn test_get_suffix_name() {
        assert_eq!(Some("obj".to_string()), get_suffix_name("aaaa.obj"));
        assert_eq!(Some("obj".to_string()), get_suffix_name("c://aaaa.obj"));
    }
}

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

use crate::{
    components::{CheckNode, FileDescriptor, ModifyPickedFile, PickedFiles, VerticeNodes},
    resources::CheckSeqence,
    system::effect_check_node,
};

#[derive(Resource)]
pub struct UIState {
    vertex_index: String,
}

impl Default for UIState {
    fn default() -> Self {
        UIState {
            vertex_index: String::from(""),
        }
    }
}

pub fn ui_system<'a>(
    mut contexts: EguiContexts,
    mut picked_files_query: Query<&mut PickedFiles>,
    mut commands: Commands,
    mut wireframe_config: ResMut<WireframeConfig>,
    mut check_seqence: ResMut<CheckSeqence>,
    mut checknode_query: Query<(Entity, &'a mut CheckNode, &Handle<StandardMaterial>)>,
    mut ui_state: ResMut<UIState>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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
        ui.horizontal(|ui| {
            ui.label("selected vertex index: ");
            ui.text_edit_singleline(&mut ui_state.vertex_index);
            if ui.button("Do Label").clicked() {
                match &ui_state.vertex_index.parse::<u32>() {
                    Ok(vertex_index) => {
                        let mut check_nodes = checknode_query
                            .iter_mut()
                            .map(|(_entity, mut node, handle_material)| (node, handle_material))
                            .collect::<Vec<(Mut<CheckNode>, &Handle<StandardMaterial>)>>();

                        if (check_nodes.len() > *vertex_index as usize) {
                            info!("选中第{}节点", vertex_index);
                            let (node, handler) = &mut check_nodes[*vertex_index as usize];
                            effect_check_node(
                                node.as_mut(),
                                handler,
                                &mut materials,
                                &mut check_seqence,
                            );
                        }
                        ui_state.vertex_index = String::from("");
                    }
                    Err(err) => {
                        info!("选中节点错误");
                        ui_state.vertex_index = String::from("");
                    }
                }
            };
        });
        ui.separator();
        if ui.button("Save").clicked() {
            if picked_files.picked_folder_path.is_some() && picked_files.current_index.is_some() {
                let v = checknode_query
                    .iter()
                    .map(|(_entity, node, _)| node)
                    .collect::<Vec<&CheckNode>>();

                // let (node_entity, _) = nodes_query
                //     .get(picked_files.current_entity.as_ref().unwrap().clone())
                //     .unwrap();
                // let (_, children) = child_query.get(node_entity).unwrap();
                // let mut v = vec![];
                // for child in children.iter() {
                //     if let Ok((_, checkNode)) = checknode_query.get(child.clone()) {
                //         v.push(checkNode)
                //     }
                // }
                info!("node length {:?}", v.len());
                let path = picked_files.files[picked_files.current_index.unwrap()]
                    .path
                    .to_string();
                let json = serde_json::to_string(check_seqence.as_ref()).unwrap();
                // let _task = IoTaskPool::get().spawn(async move {
                let path = format!("{}.json", &path);
                info!("write path {:?}", path);
                File::create(path)
                    .and_then(|mut file| file.write(json.as_bytes()))
                    .expect("Error Save Failed");
                // });
                // commands.spawn(_task);
            }
        }
        ui.separator();

        ui.collapsing("Click to see what is hidden!", |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                // ui.set_max_height(500.);
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

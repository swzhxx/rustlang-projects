use std::sync::Arc;

use bevy::{
    prelude::{
        App, AssetServer, BuildChildren, ButtonBundle, Changed, Color, Commands, Component,
        DespawnRecursiveExt, Entity, NodeBundle, Plugin, Query, Res, ResMut, State, System,
        SystemSet, TextBundle, With,
    },
    text::{Text, TextStyle},
    ui::{AlignSelf, FlexDirection, Interaction, JustifyContent, Size, Style, UiRect, Val},
    utils::default,
};

use crate::GameState;

#[derive(Component)]
pub struct MenuUIRoot;

#[derive(Component)]
pub struct StartButton;
#[derive(Component)]
pub struct QuitButton;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(spawn_main_menu))
            .add_system_set(
                SystemSet::on_update(GameState::MainMenu)
                    .with_system(start_button_clicked)
                    .with_system(quit_button_clicked),
            );
    }
}

fn start_button_clicked(
    mut commands: Commands,
    interactions: Query<&Interaction, (With<StartButton>, Changed<Interaction>)>,
    menu_root: Query<Entity, With<MenuUIRoot>>,
    mut game_state: ResMut<State<GameState>>,
) {
    for interaction in &interactions {
        if matches!(interaction, Interaction::Clicked) {
            let root_entity = menu_root.single();
            commands.entity(root_entity).despawn_recursive();
            game_state.set(GameState::GamePlay).unwrap();
        }
    }
}

fn quit_button_clicked() {}

fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let start_button = spawn_button(&mut commands, &asset_server, "StartGame", Color::Read);
    commands.entity(start_button).insert(StartButton);
    let quit_button = spawn_button(&mut commands, &asset_server, "Quit", Color::Blue);
    commands.entity(quit_button).insert(QuitButton);

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },

            ..default()
        })
        .insert(MenuUIRoot)
        .with_children(|commands| {
            commands.spawn(TextBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    margin: UiRect::all(Val::Percent(3.)),
                    ..default()
                },
                text: Text::from_section(
                    "Tower Defense Tutorial",
                    TextStyle {
                        font: asset_server.load("FiraSans-Bold.ttf"),
                        font_size: 96.0,
                        color: Color::BLACK,
                    },
                ),
                ..default()
            });
        })
        .add_child(start_button)
        .add_child(quit_button);
}

fn spawn_button(
    commands: &mut Commands,
    asset_server: &AssetServer,
    text: &str,
    color: Color,
) -> Entity {
    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Percent(65.), Val::Percent(15.)),
                align_self: AlignSelf::Center,
                justify_content: JustifyContent::Center,
                margin: UiRect::all(Val::Percent(2.)),
                ..default()
            },
            background_color: color.into(),
            ..default()
        })
        .with_children(|commands| {
            commands.spawn(TextBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    margin: UiRect::all(Val::Percent(3.)),
                    ..default()
                },
                text: Text::from_section(
                    text,
                    TextStyle {
                        font: asset_server.load("FiraSans-Bold.ttf"),
                        font_size: 64.0,
                        color: Color::BLACK,
                    },
                ),
                ..default()
            });
        })
        .id()
}

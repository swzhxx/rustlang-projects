use crate::consts::*;
use crate::types::*;
use bevy::prelude::*;
/// Keeps the textures and materials for Arrows
struct ArrowMaterialResource {
    red_texture: Handle<ColorMaterial>,
    blue_texture: Handle<ColorMaterial>,
    green_texture: Handle<ColorMaterial>,
    border_texture: Handle<ColorMaterial>,
}
impl FromWorld for ArrowMaterialResource {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();

        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        let asset_server = world.get_resource::<AssetServer>().unwrap();

        let red_handle = asset_server.load("images/arrow_red.png");
        let blue_handle = asset_server.load("images/arrow_blue.png");
        let green_handle = asset_server.load("images/arrow_green.png");
        let border_handle = asset_server.load("images/arrow_border.png");
        ArrowMaterialResource {
            red_texture: materials.add(red_handle.into()),
            blue_texture: materials.add(blue_handle.into()),
            green_texture: materials.add(green_handle.into()),
            border_texture: materials.add(border_handle.into()),
        }
    }
}

/// Actual component that goes on the sprites
struct Arrow {
    speed: Speed,
    direction: Directions,
}

struct SpawnTimer(Timer);

/// Spawns arrows
fn spawn_arrows(
    mut commands: Commands,
    mut song_config: ResMut<SongConfig>,
    materials: Res<ArrowMaterialResource>,
    time: Res<Time>,
) {
    // We get the current time since startup (secs) and the time since the last iteration (secs_last),
    // this way we check if any arrows should spawn in this window

    // Song starts 3 seconds after start, so we subtract 3 seconds
    let secs = time.seconds_since_startup() - 3.;
    let secs_last = secs - time.delta_seconds_f64();

    // Counter of how many arrows we need to spawn and remove from the list
    let mut remove_counter = 0;
    for arrow in &song_config.arrows {
        // List is ordered, so we can just check until an item fails
        // Check if arrow should be spawned at any point between last frame and this frame
        if secs_last < arrow.spawn_time && arrow.spawn_time < secs {
            remove_counter += 1;

            // Get the correct material according to speed
            let material = match arrow.speed {
                Speed::Slow => materials.red_texture.clone(),
                Speed::Medium => materials.blue_texture.clone(),
                Speed::Fast => materials.green_texture.clone(),
            };

            let mut transform =
                Transform::from_translation(Vec3::new(SPAWN_POSITION, arrow.direction.y(), 1.));
            // Rotate the arrow acording to direction
            transform.rotate(Quat::from_rotation_z(arrow.direction.rotation()));
            commands
                .spawn_bundle(SpriteBundle {
                    material,
                    sprite: Sprite::new(Vec2::new(140., 140.)),
                    transform,
                    ..Default::default()
                })
                .insert(Arrow {
                    speed: arrow.speed,
                    direction: arrow.direction,
                });
        } else {
            break;
        }
    }

    // Remove the arrows we have spawned from the list
    for _ in 0..remove_counter {
        song_config.arrows.remove(0);
    }
}

/// Moves the arrows forward
fn move_arrows(time: Res<Time>, mut query: Query<(&mut Transform, &Arrow)>) {
    for (mut transform, _arrow) in query.iter_mut() {
        transform.translation.x += time.delta_seconds() * 200.;
    }
}

pub struct ArrowsPlugin;
impl Plugin for ArrowsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ArrowMaterialResource>()
            .insert_resource(SpawnTimer(Timer::from_seconds(1., true)))
            .add_system(spawn_arrows.system())
            .add_system(move_arrows.system());
    }
}

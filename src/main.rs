#[allow(unused)]
use bevy::prelude::*;
use components::{Velocity, Movable};
use player::PlayerPlugin;
// use iyes_loopless::prelude::*;

mod components;
mod player;

// region: --- Asset Constants

const PLAYER_SPRITE: &str = "player_a_01.png"; 
const PLAYER_SIZE: (f32, f32) = (144., 75.);

const PLAYER_LASER_SPRITE: &str = "laser_a_01.png"; 
const PLAYER_LASER_SIZE: (f32, f32) = (9., 54.);

const SPRITE_SCALE: f32 = 0.5;

// endregion: --- Asset Constants

// region: --- Game Constants

const TIME_STEP: f32 = 1.0 / 60.;
const BASE_SPEED: f32 = 400.;

// endregion: --- Game Constants

// region: --- Resources 
#[derive(Resource)]
pub struct WinSize {
    pub w: f32,
    pub h: f32
}

#[derive(Resource)]
struct GameTextures {
    player: Handle<Image>,
    player_laser: Handle<Image>,
}
// endregion: --- Resources

fn main() {
    let window_plugin = WindowPlugin {
        window: WindowDescriptor {                
          width: 598.0,
          height: 676.0,
          title: "Rust Invaders".to_string(),
          ..default()
        },
        ..default()
      };
      
    let asset_plugin = AssetPlugin {
        watch_for_changes: true,
        ..default()
    };

    let default_plugins = DefaultPlugins
        .set(window_plugin)
        .set(asset_plugin);

    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(default_plugins)
        .add_startup_system(setup_system)        
        .add_system(bevy::window::close_on_esc)
        .add_system(movable_system)        
        .add_plugin(PlayerPlugin)
        .run();
}

fn setup_system(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut windows: ResMut<Windows>
) {
    // camera
    commands.spawn(Camera2dBundle::default());

    // capture window size
    let window = windows.get_primary_mut().unwrap();
    let (win_w, win_h) = (window.width(), window.height());

    // position window (for tutorial)
    // window.set_position(MonitorSelection::Primary, IVec2::new(1964 /2, 3024 / 2));

    // add WinSize resource
    let win_size = WinSize { w: win_w, h: win_h};
    commands.insert_resource(win_size);

    // add GameTexture resource
    let game_textures = GameTextures {
        player: asset_server.load(PLAYER_SPRITE),
        player_laser: asset_server.load(PLAYER_LASER_SPRITE),
    };

    commands.insert_resource(game_textures);
}

fn movable_system(
    mut commands: Commands,
    win_size: Res<WinSize>,
    mut query: Query<(Entity, &Velocity, &mut Transform, &Movable)>,
) {
    for (entity, velocity, mut transform, movable) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        translation.y += velocity.y * TIME_STEP * BASE_SPEED;
    }
}

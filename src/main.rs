#[allow(unused)]

// use iyes_loopless::prelude::*;
use bevy::prelude::*;

// region: --- Asset Constants

const PLAYER_SPRITE: &str = "player_a_01.png"; 
const PLAYER_SIZE: (f32, f32) = (144., 75.);

// endregion: --- Asset Constants

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
        .run();
}

fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    // Rectangle
    commands.spawn(SpriteBundle {
        texture: asset_server.load(PLAYER_SPRITE),
        // sprite: Sprite {
        //     color: Color::rgb(0.25, 0.25, 0.75),
        //     custom_size: Some(Vec2::new(50.0, 100.0)),
        //     ..default()
        // },
        ..default()
    });}
    

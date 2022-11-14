#[allow(unused)]

// use iyes_loopless::prelude::*;
use bevy::prelude::*;

// region: --- Asset Constants

const PLAYER_SPRITE: &str = "player_a_01.png"; 
const PLAYER_SIZE: (f32, f32) = (144., 75.);
const SPRITE_SCALE: f32 = 0.5;

// endregion: --- Asset Constants

// region: --- Resources 
#[derive(Resource)]
pub struct WinSize {
    pub w: f32,
    pub h: f32
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
        .add_startup_system_to_stage(StartupStage::PostStartup, player_spawn_system)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup_system(
    mut commands: Commands, 
    mut windows: ResMut<Windows>
) {
    // camera
    commands.spawn(Camera2dBundle::default());

    // capture window size
    let window = windows.get_primary_mut().unwrap();
    let (win_w, win_h) = (window.width(), window.height());

    // position window (for tutorial)
    // window.set_position(MonitorSelection::Primary, IVec2::new(1964 /2, 3024 / 2));
    let win_size = WinSize { w: win_w, h: win_h};
    commands.insert_resource(win_size);
}

fn player_spawn_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    win_size: Res<WinSize>
) {
    // add player
    let bottom = -win_size.h /2.;
    let player_pos_y = bottom + PLAYER_SIZE.1 / 2. + 5.;

    commands.spawn(SpriteBundle {
        texture: asset_server.load(PLAYER_SPRITE),
        transform: Transform {
            translation: Vec3::new(0., player_pos_y, 10.),
            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
            ..default()
        },
        ..default()
    });
}
    

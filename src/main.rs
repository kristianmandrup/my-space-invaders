#[allow(unused)]
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use components::{Velocity, Movable, SpriteSize, FromPlayer, Laser, FromEnemy, Enemy};
use player::PlayerPlugin;
use enemy::EnemyPlugin;
// use iyes_loopless::prelude::*;

mod components;
mod player;
mod enemy;

// region: --- Asset Constants

const PLAYER_SPRITE: &str = "player_a_01.png"; 
const PLAYER_SIZE: (f32, f32) = (144., 75.);

const PLAYER_LASER_SPRITE: &str = "laser_a_01.png"; 
const PLAYER_LASER_SIZE: (f32, f32) = (9., 54.);

const ENEMY_SPRITE: &str = "enemy_a_01.png"; 
const ENEMY_SIZE: (f32, f32) = (144., 75.);
const ENEMY_LASER_SPRITE: &str = "laser_b_01.png"; 
const ENEMY_LASER_SIZE: (f32, f32) = (7., 55.);

const SPRITE_SCALE: f32 = 0.5;

// endregion: --- Asset Constants

// region: --- Game Constants

const TIME_STEP: f32 = 1.0 / 60.;
const BASE_SPEED: f32 = 300.;

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
    enemy: Handle<Image>,
    enemy_laser: Handle<Image>,
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
        .add_system(player_laser_hit_enemy_system)   
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
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
        enemy: asset_server.load(ENEMY_SPRITE),
        enemy_laser: asset_server.load(ENEMY_LASER_SPRITE),
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

        if movable.auto_despawn {
            // despawn when out of screen
            const MARGIN: f32 = 200.;
            let outside_bottom = translation.y > win_size.h / 2. + MARGIN;
            let outside_top = translation.y < -win_size.h / 2. - MARGIN;
            let outside_right = translation.x > win_size.w / 2. + MARGIN;
            let outside_left = translation.x < -win_size.w / 2. - MARGIN;
            let outside = outside_bottom || outside_top || outside_right || outside_left;

            if outside {
                // println!("==> despawn {entity:?}");
                commands.entity(entity).despawn();
            }
        }
    }
}

fn player_laser_hit_enemy_system(
    mut commands: Commands,
    laser_query: Query<(Entity, &Transform, &SpriteSize, (With<Laser>, With<FromPlayer>))>,
    enemy_query: Query<(Entity, &Transform, &SpriteSize, (With<Enemy>))>
) {
    for (laser_entity, laser_tf, laser_size, _) in laser_query.iter() {
        let laser_scale = Vec2::new(laser_tf.scale.x, laser_tf.scale.y);

        for (enemy_entity, enemy_tf, enemy_size, _) in enemy_query.iter() {
            let enemy_scale = Vec2::new(enemy_tf.scale.x, enemy_tf.scale.y);

            // determine collision
            let collision = collide(
                laser_tf.translation, 
                laser_size.0 * laser_scale, 
                enemy_tf.translation, 
                enemy_size.0 * enemy_scale,
            );
            //perform collision logic
            if let Some(_) = collision {
                // remove enemy
                commands.entity(enemy_entity).despawn();

                // remove laser
                commands.entity(laser_entity).despawn();
            }
        }
    }
}
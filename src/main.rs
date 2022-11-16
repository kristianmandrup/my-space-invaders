use std::collections::HashSet;

#[allow(unused)]
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use components::{Velocity, Movable, SpriteSize, FromPlayer, Laser, FromEnemy, Enemy, ExplosionToSpawn, Explosion, ExplosionTimer, Player};
use player::PlayerPlugin;
use enemy::EnemyPlugin;
// use iyes_loopless::prelude::*;

mod components;
mod player;
mod enemy;

// region: --- Asset Constants

const PLAYER_SPRITE: &str = "player_a_01.png"; 
const ENEMY_SPRITE: &str = "enemy_a_01.png"; 
const EXPLOSION_SHEET: &str = "explo_a_sheet.png"; 
const EXPLOSION_LEN: usize = 16; 

const PLAYER_SIZE: (f32, f32) = (144., 75.);
const ENEMY_SIZE: (f32, f32) = (144., 75.);

const PLAYER_LASER_SPRITE: &str = "laser_a_01.png"; 
const PLAYER_LASER_SIZE: (f32, f32) = (9., 54.);

const ENEMY_LASER_SPRITE: &str = "laser_b_01.png"; 
const ENEMY_LASER_SIZE: (f32, f32) = (7., 55.);
const ENEMY_MAX_COUNT: u32 = 5;

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
    explosion: Handle<TextureAtlas>,
}

#[derive(Resource)]
struct EnemyCount(u32);

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
        .add_system(explosion_to_spawn_system)
        .add_system(explosion_animation_system)
        .add_system(enemy_laser_hit_system)  
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .run();
}

fn setup_system(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
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

    // create explosion texture atlas
    let texture_handle = asset_server.load(EXPLOSION_SHEET);
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64., 64.), 4, 4, None, None);
    let explosion = texture_atlases.add(texture_atlas);

    // add GameTexture resource
    let game_textures = GameTextures {
        player: asset_server.load(PLAYER_SPRITE),
        player_laser: asset_server.load(PLAYER_LASER_SPRITE),
        enemy: asset_server.load(ENEMY_SPRITE),
        enemy_laser: asset_server.load(ENEMY_LASER_SPRITE),
        explosion
    };

    commands
        .insert_resource(game_textures);

    commands
        .insert_resource(EnemyCount(0));

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
                println!("==> despawn entity {entity:?}");
                commands.entity(entity).despawn();
            }
        }
    }
}

fn enemy_laser_hit_system(
    mut commands: Commands,
    laser_query: Query<(Entity, &Transform, &SpriteSize), (With<Laser>, With<FromEnemy>)>,
    player_query: Query<(Entity, &Transform, &SpriteSize), With<Player>>,
)
{
    for (player_entity, player_tf, player_size) in player_query.iter() {
        let player_scale = Vec2::from((player_tf.scale.x, player_tf.scale.y));
        for (laser_entity, laser_tf, laser_size) in laser_query.iter() {
            let laser_scale = Vec2::from((laser_tf.scale.x, laser_tf.scale.y));

            // determine if collision
            let collision = collide(
                laser_tf.translation,
                laser_size.0 * laser_scale,
                player_tf.translation,
                player_size.0 * player_scale,
            );
            
            // perform collision
            if let Some(_) = collision {
                // remove player
                commands.entity(player_entity).despawn();

                // remove laser
                commands.entity(laser_entity).despawn();

                // spawn the explosionToSpawn
                commands.spawn_empty().insert(ExplosionToSpawn(player_tf.translation.clone()));

                break;
            }
        }
    }
}


fn player_laser_hit_enemy_system(
    mut commands: Commands,
    mut enemy_count: ResMut<EnemyCount>,
    laser_query: Query<(Entity, &Transform, &SpriteSize), (With<Laser>, With<FromPlayer>)>,
    enemy_query: Query<(Entity, &Transform, &SpriteSize), With<Enemy>>
) {
    let mut despawned_entities: HashSet<Entity> = HashSet::new();

    for (laser_entity, laser_tf, laser_size) in laser_query.iter() {
        if despawned_entities.contains(&laser_entity) {
            continue
        }

        let laser_scale = Vec2::new(laser_tf.scale.x, laser_tf.scale.y);

        for (enemy_entity, enemy_tf, enemy_size) in enemy_query.iter() {
            if despawned_entities.contains(&enemy_entity) || despawned_entities.contains(&laser_entity) {
                continue
            }
    
            let enemy_scale = Vec2::new(enemy_tf.scale.x, enemy_tf.scale.y);

            // determine collision
            let collision = collide(
                laser_tf.translation, 
                laser_size.0 * laser_scale, 
                enemy_tf.translation, 
                enemy_size.0 * enemy_scale,
            );

            //perform collision
            if let Some(_) = collision {
                // remove enemy
                println!("==> despawn enemy {enemy_entity:?}");
                commands.entity(enemy_entity).despawn();
                despawned_entities.insert(enemy_entity);
                enemy_count.0 -= 1;
                // remove laser
                println!("==> despawn laser {laser_entity:?}");
                commands.entity(laser_entity).despawn();
                despawned_entities.insert(laser_entity);

                // spawn the explosion
                commands.spawn_empty().insert(ExplosionToSpawn(enemy_tf.translation.clone()));
            }
        }
    }
}

fn explosion_to_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    query: Query<(Entity, &ExplosionToSpawn)>
) {
    for (explosion_entity, explosion_to_spawn) in query.iter() {
        // spawn the explosion sprite
        commands
            .spawn(SpriteSheetBundle {
                texture_atlas: game_textures.explosion.clone(),
                transform: Transform {
                    translation: explosion_to_spawn.0,
                    ..default()
                },
                ..default()
            })
            .insert(Explosion)
            .insert(ExplosionTimer::default());

        // despawn explosion
        commands.entity(explosion_entity).despawn();
    }
}

fn explosion_animation_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut ExplosionTimer, &mut TextureAtlasSprite), With<Explosion>>
) {
    for (entity,mut timer, mut sprite) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            sprite.index += 1; // move to next sprite
            if sprite.index >= EXPLOSION_LEN {
                commands.entity(entity).despawn()
            }

        }
    }
}
use bevy::{prelude::*};

use crate::{GameTextures, WinSize, SPRITE_SCALE, PLAYER_SIZE, components::{Player, Velocity, Movable}, TIME_STEP, BASE_SPEED};

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(StartupStage::PostStartup, player_spawn_system)
            .add_system(player_keyboad_event_system)
            .add_system(player_fire_system);
    }
}

fn player_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    win_size: Res<WinSize>
) {
    // add player
    let bottom = -win_size.h /2.;
    let player_pos_y = bottom + PLAYER_SIZE.1 / 2. + 5.;

    commands.spawn(SpriteBundle {
        texture: game_textures.player.clone(),
        transform: Transform {
            translation: Vec3::new(0., player_pos_y, 10.),
            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
            ..default()
        },
        ..default()
    })
    .insert(Player)
    .insert(Movable { auto_despawn: false })
    .insert(Velocity {
        x: 0.,
        y: 0.
    });
}

fn player_fire_system(
    mut commands: Commands,
    kb: Res<Input<KeyCode>>,
    game_textures: Res<GameTextures>,
    query: Query<&Transform, With<Player>>
) {
    if let Ok(player_tf) = query.get_single() {
        if kb.just_pressed(KeyCode::Space) {
            let (x,y) = (player_tf.translation.x, player_tf.translation.y);
            commands.spawn(SpriteBundle {
                texture: game_textures.player_laser.clone(),
                transform: Transform {
                    translation: Vec3::new(x, y, 0.),
                    scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                    ..default()
                },
                ..default()
            })
            .insert(Velocity {
                x: 0.,
                y: 1.
            })
            .insert(Movable { auto_despawn: true });
        }
    }
}

fn player_keyboad_event_system(
    kb: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>
) {
    if let Ok(mut velocity) = query.get_single_mut() {
        velocity.x = if kb.pressed(KeyCode::Left) {
            -1.
        } else if kb.pressed(KeyCode::Right) {
            1.
        } else {
            0.
        }
    }
}

    

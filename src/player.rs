use bevy::{prelude::*, time::FixedTimestep};

use crate::{GameTextures, WinSize, SPRITE_SCALE, PLAYER_SIZE, components::{Player, Velocity, Movable, FromPlayer, SpriteSize, Laser}, PLAYER_LASER_SIZE, PlayerState, PLAYER_RESPAWN_DELAY};

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(PlayerState::default())
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(0.5))
                    .with_system(player_spawn_system)

            )
            .add_system(player_keyboad_event_system)
            .add_system(player_fire_system);
    }
}

fn player_spawn_system(
    mut commands: Commands,
    mut player_state: ResMut<PlayerState>,
    time: Res<Time>,
    game_textures: Res<GameTextures>,
    win_size: Res<WinSize>
) {
    let now = time.elapsed_seconds_f64();
    let last_shot = player_state.last_shot;

    if !player_state.on && (last_shot == -1. || now > last_shot + PLAYER_RESPAWN_DELAY) {
        // add player
        let bottom = -win_size.h /2.;
        let player_pos_y = bottom + PLAYER_SIZE.1 / 2. + 5.;

        commands
            .spawn(SpriteBundle {
                texture: game_textures.player.clone(),
                transform: Transform {
                    translation: Vec3::new(0., player_pos_y, 10.),
                    scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                    ..default()
                },
                ..default()
            })
            .insert(SpriteSize::from(PLAYER_SIZE))
            .insert(Player)
            .insert(Movable { auto_despawn: false })
            .insert(Velocity {
                x: 0.,
                y: 0.
            });
        player_state.spawned()
    }
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
            let x_offset = PLAYER_SIZE.0 / 2. * SPRITE_SCALE - 5.;
            let y_offset = 15.;
            let mut spawn_laser = |x_offset: f32| {
                commands.spawn(SpriteBundle {
                    texture: game_textures.player_laser.clone(),
                    transform: Transform {
                        translation: Vec3::new(x + x_offset, y + y_offset, 0.),
                        scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                        ..default()
                    },
                    ..default()
                })
                .insert(Laser)
                .insert(FromPlayer)
                .insert(SpriteSize::from(PLAYER_LASER_SIZE))            
                .insert(Velocity {
                    x: 0.,
                    y: 1.
                })
                .insert(Movable { auto_despawn: true });    
            };
            spawn_laser(x_offset);
            spawn_laser(-x_offset);
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

    

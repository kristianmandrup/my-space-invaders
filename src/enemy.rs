use bevy::{prelude::*, time::FixedTimestep};
use rand::{thread_rng, Rng};
use crate::{GameTextures, SPRITE_SCALE, WinSize, components::{Enemy, SpriteSize}, ENEMY_LASER_SIZE, ENEMY_SIZE, ENEMY_MAX_COUNT, EnemyCount};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(2.))
                    .with_system(enemy_spawn_system)
            );
    }
}

fn enemy_spawn_system(
    mut commands: Commands, 
    mut enemy_count: ResMut<EnemyCount>,    
    game_textures: Res<GameTextures>,
    win_size: Res<WinSize>
) {
    if enemy_count.0 >= ENEMY_MAX_COUNT {
        return
    }
    // compute x,y
    let mut rng = thread_rng();
    let w_span = win_size.w / 2. - 100.;
    let h_span = win_size.h / 2. - 200.;
    let x = rng.gen_range(-w_span..w_span);
    let y = rng.gen_range(-h_span..h_span);

    commands.spawn(SpriteBundle {
        texture: game_textures.enemy.clone(),
        transform: Transform {
            translation: Vec3::new(x, y, 10.),
            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
            ..default()
        },
        ..default()
    })
    .insert(SpriteSize::from(ENEMY_SIZE))
    .insert(Enemy);
    enemy_count.0 += 1
}
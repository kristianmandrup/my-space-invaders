use std::{f32::consts::PI};
use bevy::{prelude::*, time::FixedTimestep, ecs::schedule::ShouldRun};
use rand::{thread_rng, Rng};
use crate::{GameTextures, SPRITE_SCALE, WinSize, components::{Enemy, SpriteSize, Velocity, Movable, FromEnemy, Laser}, ENEMY_LASER_SIZE, ENEMY_SIZE, ENEMY_MAX_COUNT, EnemyCount, TIME_STEP};

use self::formation::{FormationMaker, Formation};

mod formation;



pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(FormationMaker::default())
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(2.))
                    .with_system(enemy_spawn_system)
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(enemy_fire_criteria)
                    .with_system(enemy_fire_system)
            )
            .add_system(enemy_movement_system);
    }
}

fn enemy_movement_system(
    // time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Formation), With<Enemy>>,    
)
{
    // let now = time.elapsed_seconds();
    for (mut transform, mut formation) in query.iter_mut() {
        // current pos
        let (x_org, y_org) = (transform.translation.x, transform.translation.y);

        let delta = TIME_STEP * formation.speed;

        // max distance
        let max_distance = delta; 

        // fixtures
        let dir: f32 = if formation.start.0 < 0. { 1. } else { -1. }; // 1 is counter-clockwise, -1 is clockwise
        let (x_pivot, y_pivot) = formation.pivot;
        let (x_radius, y_radius) = formation.radius;

        // compute next angle (based on time)
        let angle = formation.angle + dir * formation.speed * TIME_STEP / (x_radius.min(y_radius) * PI / 2.);

        // compute target x/y
        let x_dst = x_radius * angle.cos() + x_pivot;
        let y_dst = y_radius * angle.sin() + y_pivot;

        // compute distance
        let dx = x_org - x_dst;
        let dy = y_org - y_dst;
        let distance = (dx * dx + dy * dy).sqrt();
        let distance_ratio = if distance != 0. {
            max_distance / distance
        } else {
            0.
        };
        // compute final x/y
        let x = x_org - dx * distance_ratio;
        let x = if dx > 0. { x.max(x_dst) } else { x.min(x_dst) };

        let y = y_org - dy * distance_ratio;
        let y = if dy > 0. { y.max(y_dst) } else { y.min(y_dst) };

        // start rotating the formation angle only when sprite is on or close to ellipse
        if distance < max_distance * formation.speed / 20. {
            formation.angle = angle
        }


        // new pos
        let translation = &mut transform.translation;
        (translation.x, translation.y) = (x, y);
    }
}

fn enemy_spawn_system(
    mut commands: Commands, 
    mut enemy_count: ResMut<EnemyCount>,   
    mut formation_maker: ResMut<FormationMaker>, 
    game_textures: Res<GameTextures>,
    win_size: Res<WinSize>
) {
    if enemy_count.0 >= ENEMY_MAX_COUNT {
        return
    }
    // get formation and start x/y
    let formation = formation_maker.make(&win_size);
    let (x, y) = formation.start;


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
    .insert(Enemy)
    .insert(formation);
    
    enemy_count.0 += 1
}

fn enemy_fire_criteria(
) -> ShouldRun {
    if thread_rng().gen_bool(1. / 90.) {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

fn enemy_fire_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    enemy_query: Query<&Transform, With<Enemy>>
) {
    for &tf in enemy_query.iter() {
        let (x, y) = (tf.translation.x, tf.translation.y);
        commands
            .spawn(SpriteBundle {
                texture: game_textures.enemy_laser.clone(),
                transform: Transform {
                    translation: Vec3::new(x, y - 15., 0.),
                    rotation: Quat::from_rotation_x(PI),
                    scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                    ..default()
                },
                ..default()
            })
            .insert(Laser)
            .insert(SpriteSize::from(ENEMY_LASER_SIZE))
            .insert(FromEnemy)
            .insert(Movable {
                auto_despawn: true
            })
            .insert(Velocity {
                x: 0.,
                y: -1.
            });
    }
    
}
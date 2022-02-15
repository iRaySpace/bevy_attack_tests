use bevy::{core::FixedTimestep, input::system::exit_on_esc_system, prelude::*};

const TIMESTEP: f32 = 1.0 / 60.0;

#[derive(Component, Debug)]
struct Tower {
    attack_range: f32,
    can_attack: bool,
    cooldown_seconds: f32,
    cooldown_seconds_timer: f32,
}

#[derive(Component)]
struct Enemy;

#[derive(Component, Debug)]
struct Projectile {
    target_translation: Vec3,
    speed: f32,
}

fn startup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: Vec3::new(30.0, 30.0, 0.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgb(0.95, 0.95, 0.95),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Tower {
            attack_range: 300.0,
            can_attack: true,
            cooldown_seconds: 1.0,
            cooldown_seconds_timer: 0.0,
        });
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(-90.0, 0.0, 0.0),
                scale: Vec3::new(30.0, 30.0, 0.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgb(0.95, 0.47, 0.47),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Enemy);
}

fn tower_attack_system(
    mut tower_query: Query<(&mut Tower, &Transform)>,
    enemy_query: Query<(Entity, &Enemy, &Transform)>,
    mut commands: Commands,
) {
    for (mut tower, t_transform) in tower_query.iter_mut() {
        for (enemy_entity, _enemy, e_transform) in enemy_query.iter() {
            let distance = t_transform.translation.distance(e_transform.translation);
            if distance < tower.attack_range && tower.can_attack {
                commands
                    .spawn_bundle(SpriteBundle {
                        transform: Transform {
                            translation: Vec3::new(
                                t_transform.translation.x,
                                t_transform.translation.y,
                                0.0,
                            ),
                            scale: Vec3::new(10.0, 10.0, 0.0),
                            ..Default::default()
                        },
                        sprite: Sprite {
                            color: Color::rgb(0.25, 0.25, 0.25),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Projectile {
                        target_translation: Vec3::new(-90.0, 0.0, 0.0),
                        speed: 100.0,
                    });
                tower.can_attack = false;
                tower.cooldown_seconds_timer = tower.cooldown_seconds;
            }
        }
    }
}

fn projectile_system(
    mut projectile_query: Query<(Entity, &Projectile, &mut Transform)>,
    mut commands: Commands,
) {
    for (entity, projectile, mut transform) in projectile_query.iter_mut() {
        if transform.translation.x > projectile.target_translation.x {
            transform.translation.x -= projectile.speed * TIMESTEP;
        }
        if transform.translation.x < projectile.target_translation.x {
            transform.translation.x += projectile.speed * TIMESTEP;
        }
        if transform.translation.y > projectile.target_translation.y {
            transform.translation.y -= projectile.speed * TIMESTEP;
        }
        if transform.translation.y < projectile.target_translation.y {
            transform.translation.y += projectile.speed * TIMESTEP;
        }
        if transform
            .translation
            .distance(projectile.target_translation)
            < 1.0
        {
            commands.entity(entity).despawn();
        }
    }
}

fn cooldown_system(time: Res<Time>, mut tower_query: Query<&mut Tower>) {
    for mut tower in tower_query.iter_mut() {
        if !tower.can_attack {
            if tower.cooldown_seconds_timer > 0.0 {
                tower.cooldown_seconds_timer -= time.delta_seconds();
            }
            if tower.cooldown_seconds_timer < 0.0 {
                tower.cooldown_seconds_timer = 0.0;
                tower.can_attack = true;
            }
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(startup)
        .add_system(tower_attack_system)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIMESTEP as f64))
                .with_system(projectile_system)
                .with_system(cooldown_system),
        )
        .add_system(exit_on_esc_system)
        .run();
}

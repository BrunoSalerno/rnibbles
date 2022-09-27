use bevy::prelude::*;

#[derive(Component)]
struct WormHead;

#[derive(Component)]
enum Direction {
    Up,
    Down,
    Right,
    Left
}

#[derive(Component)]
struct MovementTimer {
    timer: Timer
}

#[derive(Bundle)]
struct WormHeadBundle {
    worm_head: WormHead,
    direction: Direction,
    #[bundle]
    sprite: SpriteBundle,
    movement_timer: MovementTimer,
}

#[derive(Component)]
struct WormBodyPart;

#[derive(Component)]
struct WormBodyPartIndex(u8);

#[derive(Bundle)]
struct WormBodyPartBundle {
    worm_body_part: WormBodyPart,
    index: WormBodyPartIndex,
    #[bundle]
    sprite: SpriteBundle,
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "RNibbles".to_string(),
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(controls)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(Camera2dBundle::default());

    commands.spawn_bundle(WormHeadBundle {
        worm_head: WormHead,
        direction: Direction::Right,
        sprite: SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(255., 255., 255.),
                custom_size: Some(Vec2::new(25.0, 25.0)),
                ..default()
            },
            ..default()
        },
        movement_timer:  MovementTimer{
            timer: Timer::from_seconds(0.5, true),
        }
    });

    for n in 0..5 {
        commands.spawn_bundle(WormBodyPartBundle {
            worm_body_part: WormBodyPart,
            index: WormBodyPartIndex(n),
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.25, 0.25, 0.75),
                    custom_size: Some(Vec2::new(25.0, 25.0)),
                    ..default()
                },
                ..default()
            }
        });
    }
}

fn controls(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut query_head: Query<(&mut Direction, &mut Transform, &mut MovementTimer), (With<WormHead>, Without<WormBodyPart>)>,
    mut query_body: Query<&mut Transform, (With<WormBodyPart>, Without<WormHead>)>,
) {
    let (mut direction, mut transform, mut movement_timer) = query_head.single_mut();

    if movement_timer.timer.tick(time.delta()).finished() {
        let mut orig_x:f32 = transform.translation.x;
        let mut orig_y:f32 = transform.translation.y;
        let mut old_orig_x:f32 = 0.;
        let mut old_orig_y:f32 = 0.;

        match *direction {
            Direction::Up => transform.translation.y += 25.,
            Direction::Down => transform.translation.y -= 25.,
            Direction::Right => transform.translation.x += 25.,
            Direction::Left => transform.translation.x -= 25.,
        }

        for mut transform in &mut query_body {
            old_orig_x = transform.translation.x;
            old_orig_y = transform.translation.y;
            transform.translation.x = orig_x;
            transform.translation.y = orig_y;
            orig_x = old_orig_x;
            orig_y = old_orig_y;
        }
    }

    if keys.pressed(KeyCode::Right) {
        *direction = Direction::Right
    }
    if keys.pressed(KeyCode::Left) {
        *direction = Direction::Left
    }
    if keys.pressed(KeyCode::Down) {
        *direction = Direction::Down
    }
    if keys.pressed(KeyCode::Up) {
        *direction = Direction::Up
    }
}

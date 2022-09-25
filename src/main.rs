use bevy::prelude::*;

#[derive(Component)]
struct WormName(String);

#[derive(Component)]
enum Direction {
    Up,
    Down,
    Right,
    Left
}

#[derive(Bundle)]
struct Worm {
    name: WormName,
    direction: Direction,
    #[bundle]
    sprite: SpriteBundle,
}

#[derive(Component)]
struct WormPartIndex(u8);

#[derive(Bundle)]
struct WormPart {
    index: WormPartIndex,
    #[bundle]
    sprite: SpriteBundle,
}

fn main() {
    App::new()
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

    commands.spawn_bundle(Worm{
        name: WormName("Wormy".to_string()),
        direction: Direction::Right,
        sprite: SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(25.0, 25.0)),
                ..default()
            },
            ..default()
        }
    });

    for n in 0..5 {
        commands.spawn_bundle(WormPart{
            index: WormPartIndex(n),
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
    mut query: Query<(&WormName, &mut Direction, &mut Transform)>,
    mut query_parts: Query<(&WormPartIndex, &mut Transform), Without<WormName>>,
) {
    let mut orig_x:f32 = 0.;
    let mut orig_y:f32 = 0.;
    let mut old_orig_x:f32 = 0.;
    let mut old_orig_y:f32 = 0.;

    for (name, mut direction, mut transform) in &mut query {
        orig_x = transform.translation.x;
        orig_y = transform.translation.y;

        match *direction {
            Direction::Up => transform.translation.y += 150. * time.delta_seconds(),
            Direction::Down => transform.translation.y -= 150. * time.delta_seconds(),
            Direction::Right => transform.translation.x += 150. * time.delta_seconds(),
            Direction::Left => transform.translation.x -= 150. * time.delta_seconds(),
        }

        if keys.pressed(KeyCode::D) {
            *direction = Direction::Right
        }
        if keys.pressed(KeyCode::A) {
            *direction = Direction::Left
        }
        if keys.pressed(KeyCode::S) {
            *direction = Direction::Down
        }
        if keys.pressed(KeyCode::W) {
            *direction = Direction::Up
        }
    }

    for (index, mut transform) in &mut query_parts {
        old_orig_x = transform.translation.x;
        old_orig_y = transform.translation.y;
        transform.translation.x = orig_x;
        transform.translation.y = orig_y;
        orig_x = old_orig_x;
        orig_y = old_orig_y;
    }
}

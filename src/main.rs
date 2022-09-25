use bevy::prelude::*;

#[derive(Component)]
struct NibblesName(String);

#[derive(Component)]
enum Direction {
    Up,
    Down,
    Right,
    Left
}


#[derive(Bundle)]
struct Nibbles {
    name: NibblesName,
    direction: Direction,
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

    commands.spawn_bundle(Nibbles{
        name: NibblesName("Wormy".to_string()),
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
}

fn controls(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&NibblesName, &mut Direction, &mut Transform)>,
) {
    for (name, mut direction, mut transform) in &mut query {
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
}

use bevy::prelude::*;
use rand::Rng;

#[derive(Component, PartialEq)]
enum Direction {
    Up,
    Down,
    Right,
    Left
}

#[derive(Component)]
struct Worm {
    direction: Direction,
    timer: Timer,
    head_x: f32,
    head_y: f32,
}

#[derive(Component)]
struct WormBodyPart;

#[derive(Bundle)]
struct WormBodyPartBundle {
    worm_body_part: WormBodyPart,
    #[bundle]
    sprite: SpriteBundle,
}

#[derive(Component)]
struct Fruit;

#[derive(Bundle)]
struct FruitBundle {
    fruit: Fruit,
    #[bundle]
    sprite: SpriteBundle,
}


const WORM_BODY_SIZE:f32 = 25.;

const BOARD_COLOR:Color = Color::rgba(191./255., 238./255., 144./255., 0.3);
const BOARD_WIDTH:f32 = 875.;
const BOARD_HEIGHT:f32 = 675.;
const BOARD_MAX_X:f32 = BOARD_WIDTH / 2. - WORM_BODY_SIZE / 2.;
const BOARD_MIN_X:f32 = - BOARD_WIDTH / 2. +  WORM_BODY_SIZE / 2.;
const BOARD_MAX_Y:f32 = BOARD_HEIGHT / 2. -  WORM_BODY_SIZE / 2.;
const BOARD_MIN_Y:f32 = - BOARD_HEIGHT / 2. +  WORM_BODY_SIZE / 2.;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "RNibbles".to_string(),
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(update_worm_position)
        .add_system(controls)
        .add_system(update_label)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(Camera2dBundle::default());

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: BOARD_COLOR,
            custom_size: Some(Vec2::new(BOARD_WIDTH, BOARD_HEIGHT)),
            ..default()
        },
        ..default()
    });

    commands.spawn_bundle(
        TextBundle::from_section(
            "",
            TextStyle {
                font: asset_server.load("FiraMono-Medium.ttf"),
                font_size: 25.,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..default()
            },
            ..default()
        })
    );

    commands.spawn().insert(Worm {
        direction: Direction::Right,
        timer: Timer::from_seconds(0.5, true),
        head_x: 0.,
        head_y: 0.,
    });

    for n in 0..5 {
        let mut color = Color::rgb(0.25, 0.25, 0.75);
        if n == 0 {
            color = Color::rgb(255., 255., 255.);
        }
        commands.spawn_bundle(WormBodyPartBundle {
            worm_body_part: WormBodyPart,
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: color,
                    custom_size: Some(Vec2::new(WORM_BODY_SIZE, WORM_BODY_SIZE)),
                    ..default()
                },
                ..default()
            }
        });
    }

    let (fruit_x, fruit_y) = get_fruit_random_position();
    commands.spawn_bundle(FruitBundle {
        fruit: Fruit,
        sprite: SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::new(WORM_BODY_SIZE, WORM_BODY_SIZE)),
                ..default()
            },
            transform: Transform {
                translation: Vec3 { x: fruit_x, y: fruit_y, z: 0. },
                ..default()
            },
            ..default()
        }
    });
}

fn get_fruit_random_position() -> (f32, f32) {
    let mut rng = rand::thread_rng();
    let range_x:f32 = (BOARD_MAX_X - BOARD_MIN_X) / 25.;
    let range_y:f32 = (BOARD_MAX_Y - BOARD_MIN_Y) / 25.;
    let x_in_range:f32 = (rng.gen_range(0..range_x as u32) as f32 * 25.);
    let y_in_range:f32 = (rng.gen_range(0..range_y as u32) as f32 * 25.);
    let x:f32 = BOARD_MIN_X + x_in_range;
    let y:f32 = BOARD_MIN_Y + y_in_range;
    (x, y)
}

fn update_worm_position(
    time: Res<Time>,
    mut query_worm: Query<&mut Worm>,
    mut query_body: Query<&mut Transform, With<WormBodyPart>>,
    mut query_fruit: Query<&mut Transform, (With<Fruit>, Without<WormBodyPart>)>
) {
    let mut worm = query_worm.single_mut();
    let mut fruit_transform = query_fruit.single_mut();

    if worm.timer.tick(time.delta()).finished() {
        match worm.direction {
            Direction::Up => worm.head_y += WORM_BODY_SIZE,
            Direction::Down => worm.head_y -= WORM_BODY_SIZE,
            Direction::Right => worm.head_x += WORM_BODY_SIZE,
            Direction::Left => worm.head_x -= WORM_BODY_SIZE,
        }

        if worm.head_x == fruit_transform.translation.x && worm.head_y == fruit_transform.translation.y {
            let (fruit_x, fruit_y) = get_fruit_random_position();
            fruit_transform.translation.x = fruit_x;
            fruit_transform.translation.y = fruit_y;
        }

        if worm.head_x < BOARD_MIN_X {
            worm.head_x = BOARD_MAX_X;
        }
        if worm.head_x > BOARD_MAX_X {
            worm.head_x = BOARD_MIN_X;
        }
        if worm.head_y < BOARD_MIN_Y {
            worm.head_y = BOARD_MAX_Y;
        }
        if worm.head_y > BOARD_MAX_Y {
            worm.head_y = BOARD_MIN_Y;
        }

        let mut orig_x:f32 = worm.head_x;
        let mut orig_y:f32 = worm.head_y;
        let mut old_orig_x:f32 = 0.;
        let mut old_orig_y:f32 = 0.;

        for mut transform in &mut query_body {
            old_orig_x = transform.translation.x;
            old_orig_y = transform.translation.y;
            transform.translation.x = orig_x;
            transform.translation.y = orig_y;
            orig_x = old_orig_x;
            orig_y = old_orig_y;
        }
    }
}

fn controls(
    keys: Res<Input<KeyCode>>,
    mut query_worm: Query<&mut Worm>,
) {
    let mut worm = query_worm.single_mut();

    if keys.pressed(KeyCode::Right) && worm.direction != Direction::Left {
        worm.direction = Direction::Right;
    }
    if keys.pressed(KeyCode::Left) && worm.direction != Direction::Right {
        worm.direction = Direction::Left;
    }
    if keys.pressed(KeyCode::Down) && worm.direction != Direction::Up {
        worm.direction = Direction::Down;
    }
    if keys.pressed(KeyCode::Up) && worm.direction != Direction::Down {
        worm.direction = Direction::Up;
    }
}

fn update_label(
    query_worm: Query<&mut Worm>,
    mut query_text: Query<&mut Text>,
) {
    let worm = query_worm.single();
    let mut text = query_text.single_mut();
    text.sections[0].value = worm.head_x.to_string() + &','.to_string() + &worm.head_y.to_string();
}

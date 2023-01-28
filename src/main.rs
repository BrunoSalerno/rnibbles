use bevy::prelude::*;
use rand::Rng;
use random_color::RandomColor;
use std::f32::consts::PI;

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
    timer_duration: f32,
    timer: Timer,
    head_x: f32,
    head_y: f32,
    level: u8,
    max_level_reached: u8,
    parts: Vec<Entity>,
    body_handle: Handle<Mesh>,
    head_color_handle: Handle<StandardMaterial>,
    body_color_handle: Handle<StandardMaterial>,
}

#[derive(Component)]
struct WormBodyPart;

#[derive(Bundle)]
struct WormBodyPartBundle {
    worm_body_part: WormBodyPart,
    #[bundle]
    pbr: PbrBundle,
}

#[derive(Component)]
struct Fruit;

#[derive(Bundle)]
struct FruitBundle {
    fruit: Fruit,
    #[bundle]
    pbr: PbrBundle,
}

const WORM_HEAD_COLOR:Color = Color::WHITE;
const WORM_BODY_COLOR:Color = Color::rgb(0.25, 0.25, 0.75);
const WORM_BODY_SIZE:f32 = 25.;
const FRUIT_RADIUS:f32 = 12.5;

const BOARD_COLOR:Color = Color::rgba(0.75, 0.93, 0.56, 0.3);
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
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // directional 'sun' light
    const HALF_SIZE: f32 = 500.0;
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            // Configure the projection to better fit the scene
            shadow_projection: OrthographicProjection {
                left: -HALF_SIZE,
                right: HALF_SIZE,
                bottom: -HALF_SIZE,
                top: HALF_SIZE,
                near: -10.0 * HALF_SIZE,
                far: 10.0 * HALF_SIZE,
                ..default()
            },
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 300.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        ..default()
    });

    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(0.0, -700.0, 450.0).looking_at(Vec3::ZERO, Vec3::Z),
        ..default()
    });

    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Box {
            max_x: BOARD_MAX_X,
            min_x: BOARD_MIN_X,
            max_y: BOARD_MAX_Y,
            min_y: BOARD_MIN_Y,
            max_z: 0.,
            min_z: -50.,
        })),
        material: materials.add(BOARD_COLOR.into()),
        ..default()
    });

    commands.spawn_bundle(
        TextBundle::from_sections([
            TextSection::new(
                "",
                TextStyle {
                    font: asset_server.load("FiraMono-Medium.ttf"),
                    font_size: 25.,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            TextSection::new(
                "",
                TextStyle {
                    font: asset_server.load("FiraMono-Medium.ttf"),
                    font_size: 25.,
                    color: Color::RED,
                    ..default()
                },
            ),
        ])
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
        timer_duration: 0.5,
        timer: Timer::from_seconds(0.5, true),
        head_x: 0.,
        head_y: 0.,
        level: 1,
        max_level_reached: 1,
        parts: Vec::new(),
        body_handle: meshes.add(Mesh::from(shape::Cube { size: WORM_BODY_SIZE })),
        head_color_handle: materials.add(WORM_HEAD_COLOR.into()),
        body_color_handle: materials.add(WORM_BODY_COLOR.into()),
    });

    let (color, fruit_x, fruit_y) = get_fruit_data();
    commands.spawn_bundle(FruitBundle {
        fruit: Fruit,
        pbr: PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere { radius: FRUIT_RADIUS, subdivisions: 2 })),
            material: materials.add(color.into()),
            transform: Transform {
                translation: Vec3 { x: fruit_x, y: fruit_y, z: 12.5 },
                ..default()
            },
            ..default()
        }
    });
}

fn get_color_from_urgba(r: u8, g: u8, b: u8, a:f32) -> Color {
    Color::rgba(r as f32 / 255., g as f32 / 255., b as f32 / 255., a)
}

fn get_fruit_data() -> (Color, f32, f32) {
    let mut rng = rand::thread_rng();
    let range_x:f32 = (BOARD_MAX_X - BOARD_MIN_X) / 25.;
    let range_y:f32 = (BOARD_MAX_Y - BOARD_MIN_Y) / 25.;
    let x_in_range:f32 = rng.gen_range(0..range_x as u32) as f32 * 25.;
    let y_in_range:f32 = rng.gen_range(0..range_y as u32) as f32 * 25.;
    let [r, g, b] = RandomColor::new().to_rgb_array();
    let color = get_color_from_urgba(r, g, b, 1.);
    (color, BOARD_MIN_X + x_in_range, BOARD_MIN_Y + y_in_range)
}

fn update_worm_position(
    mut commands: Commands,
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
        let mut old_orig_x:f32;
        let mut old_orig_y:f32;

        for (i, mut transform) in query_body.iter_mut().enumerate() {
            if i > 0 && orig_x == worm.head_x && orig_y == worm.head_y {
                // If we touch other part of the worm, we reset everything
                worm.level = 1;
                worm.timer_duration = 0.5;
                worm.timer = Timer::from_seconds(worm.timer_duration, true);
                for entity in &worm.parts {
                    commands.entity(*entity).despawn();
                }
                worm.parts = Vec::new();
            }
            old_orig_x = transform.translation.x;
            old_orig_y = transform.translation.y;
            transform.translation.x = orig_x;
            transform.translation.y = orig_y;
            orig_x = old_orig_x;
            orig_y = old_orig_y;
        }

        // Initial part of the body
        if worm.parts.len() < 5 {
            let is_head:bool = worm.parts.len() == 0;
            let entity_id = commands.spawn_bundle(get_worm_body_part(is_head, &worm, orig_x, orig_y)).id();
            worm.parts.push(entity_id);
        }
        if worm.head_x == fruit_transform.translation.x && worm.head_y == fruit_transform.translation.y {
            let (color, fruit_x, fruit_y) = get_fruit_data();
            // fruit_sprite.color = color;
            fruit_transform.translation.x = fruit_x;
            fruit_transform.translation.y = fruit_y;
            worm.timer_duration = 0.9 * worm.timer_duration;
            worm.timer = Timer::from_seconds(worm.timer_duration, true);
            worm.level += 1;
            if worm.level > worm.max_level_reached {
                worm.max_level_reached = worm.level;
            }
            // we make the worm longer
            let entity_id = commands.spawn_bundle(get_worm_body_part(false, &worm, orig_x, orig_y)).id();
            worm.parts.push(entity_id);
        }
    }
}

fn controls(keys: Res<Input<KeyCode>>, mut query_worm: Query<&mut Worm>) {
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

fn update_label(query_worm: Query<&mut Worm>, mut query_text: Query<&mut Text>) {
    let worm = query_worm.single();
    let mut text = query_text.single_mut();

    text.sections[0].value = String::from("Level ") + &worm.level.to_string();

    let mut max_level_reached = String::from("");
    if worm.max_level_reached > worm.level {
        max_level_reached = String::from(" Max: ") + &worm.max_level_reached.to_string();
    }
    text.sections[1].value = max_level_reached;
}

fn get_worm_body_part(
    is_head:bool,
    worm:&Worm,
    x: f32,
    y: f32,
    ) -> WormBodyPartBundle {
    let color_handle:&Handle<StandardMaterial> = if is_head { &worm.head_color_handle } else { &worm.body_color_handle };
    WormBodyPartBundle {
        worm_body_part: WormBodyPart,
        pbr: PbrBundle {
            mesh: worm.body_handle.clone(),
            material: color_handle.clone(),
            transform: Transform {
                translation: Vec3 { x: x, y: y, z: 12.5 },
                ..default()
            },
            ..default()
        }
    }
}

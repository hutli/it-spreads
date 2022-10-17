use bevy::{prelude::*, render::render_resource::encase::rts_array::Length};
use rand::prelude::*;

const BACKGROUND_COLOR: Color =
    Color::rgb(0x87 as f32 / 256., 0xce as f32 / 256., 0xeb as f32 / 256.);
const DIRT_COLORS: [Color; 5] = [
    Color::rgb(0x40 as f32 / 256., 0x29 as f32 / 256., 0x05 as f32 / 256.),
    Color::rgb(0x76 as f32 / 256., 0x55 as f32 / 256., 0x2b as f32 / 256.),
    Color::rgb(0x6b as f32 / 256., 0x54 as f32 / 256., 0x28 as f32 / 256.),
    Color::rgb(0xb6 as f32 / 256., 0x9f as f32 / 256., 0x66 as f32 / 256.),
    Color::rgb(0xea as f32 / 256., 0xd0 as f32 / 256., 0xa8 as f32 / 256.),
];

const BRICK_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
const SPECK_SIZE: Vec2 = Vec2::new(1., 1.);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(setup)
        .run();
}

#[derive(Component)]
struct Collider;

#[derive(Component)]
struct Speck;

fn setup(mut commands: Commands, windows: Res<Windows>) {
    for window in windows.iter() {
        let left_border: i32 = -(window.physical_width() as i32) / 2;
        let right_border: i32 = (window.physical_width() as i32) / 2;
        let bottom_border: i32 = -(window.physical_height() as i32) / 2;
        let top_border: i32 = (window.physical_height() as i32) / 2;

        // Camera
        commands.spawn_bundle(Camera2dBundle::default());
        // brick
        let mut height = 200;
        let mut rng = rand::thread_rng();
        let color_borders = (-bottom_border + top_border) / 5;
        for x in left_border..right_border {
            height = height + rng.gen::<i32>() % 2;
            for y in bottom_border..height {
                let brick_position = Vec2::new(x as f32, y as f32);
                commands
                    .spawn()
                    .insert(Speck)
                    .insert_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: DIRT_COLORS[(((-bottom_border + y) / color_borders)
                                % (DIRT_COLORS.length() as i32))
                                as usize],
                            ..default()
                        },
                        transform: Transform {
                            translation: brick_position.extend(0.0),
                            scale: Vec3::new(SPECK_SIZE.x, SPECK_SIZE.y, 1.0),
                            ..default()
                        },
                        ..default()
                    })
                    .insert(Collider);
            }
        }
    }
}

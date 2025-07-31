use bevy::prelude::*;

const TILE_SIZE: f32 = 32.0;
const MAP_WIDTH: usize = 10;
const MAP_HEIGHT: usize = 10;

// 간단한 타일 타입
#[derive(Copy, Clone)]
enum TileType {
    Wall,
    Floor,
}

// 타일맵 정의 (0 = Wall, 1 = Floor)
const MAP: [[u8; MAP_WIDTH]; MAP_HEIGHT] = [
    [0,0,0,0,0,0,0,0,0,0],
    [0,1,1,1,1,1,1,1,1,0],
    [0,1,0,0,1,0,0,0,1,0],
    [0,1,0,0,1,0,0,0,1,0],
    [0,1,1,1,1,1,1,0,1,0],
    [0,1,0,0,0,0,1,0,1,0],
    [0,1,1,1,1,1,1,0,1,0],
    [0,1,0,0,1,0,0,0,1,0],
    [0,1,1,1,1,1,1,1,1,0],
    [0,0,0,0,0,0,0,0,0,0],
];

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_camera, spawn_map))
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_map(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let tile = match MAP[y][x] {
                0 => TileType::Wall,
                1 => TileType::Floor,
                _ => panic!("Unknown tile type"),
            };

            let color = match tile {
                TileType::Wall => Color::BLUE,
                TileType::Floor => Color::BLACK,
            };

            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::splat(TILE_SIZE - 2.0)), // 간격 살짝 띄우기
                    ..default()
                },
                transform: Transform::from_xyz(
                    (x as f32 - MAP_WIDTH as f32 / 2.0) * TILE_SIZE,
                    (-(y as f32) + MAP_HEIGHT as f32 / 2.0) * TILE_SIZE,
                    0.0,
                ),
                ..default()
            });
        }
    }
}

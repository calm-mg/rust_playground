use bevy::prelude::*;
use std::collections::HashSet;

use crate::game::components::*;
use crate::game::ui::HudText;

const W: usize = MAP_WIDTH;
const H: usize = MAP_HEIGHT;

// 0: Wall, 1: Floor+pellet, 2: PowerPellet, 9: Floor only
// 대충 팩맨 비슷한 레이아웃. 가장자리는 벽, 모서리에 파워펠릿.
const MAP: [[u8; W]; H] = {
    [
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,2,1,1,1,1,1,1,1,0,1,1,1,1,1,1,1,2,0],
        [0,1,0,0,1,0,0,0,1,0,1,0,0,0,1,0,0,1,0],
        [0,1,1,1,1,1,1,0,1,0,1,0,1,1,1,1,1,1,0],
        [0,1,0,0,1,0,1,0,1,0,1,0,1,0,1,0,0,1,0],
        [0,1,1,1,1,0,1,1,1,9,1,1,1,0,1,1,1,1,0],
        [0,0,0,0,1,0,0,0,0,9,0,0,0,0,1,0,0,0,0],
        [0,1,1,1,1,1,1,0,1,9,1,0,1,1,1,1,1,1,0],
        [0,1,0,0,1,0,1,0,1,0,1,0,1,0,1,0,0,1,0],
        [0,1,1,1,1,0,1,1,1,9,1,1,1,0,1,1,1,1,0],
        [0,0,0,0,1,0,0,0,1,0,1,0,0,0,1,0,0,0,0],
        [0,1,1,1,1,1,1,0,1,9,1,0,1,1,1,1,1,1,0],
        [0,1,0,0,1,0,1,0,1,0,1,0,1,0,1,0,0,1,0],
        [0,1,1,1,1,0,1,1,1,9,1,1,1,0,1,1,1,1,0],
        [0,0,0,0,1,0,0,0,0,9,0,0,0,0,1,0,0,0,0],
        [0,1,1,1,1,0,1,1,1,9,1,1,1,0,1,1,1,1,0],
        [0,1,0,0,1,0,1,0,1,0,1,0,1,0,1,0,0,1,0],
        [0,2,1,1,1,1,1,1,1,9,1,1,1,1,1,1,1,2,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    ]
};

pub fn setup_world(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut wallset: ResMut<WallSet>,
    mut pellet_count: ResMut<PelletCount>,
) {
    
    let mut cam = Camera2dBundle::default();
    cam.transform.translation = Vec3::new(0.0, 0.0, 1000.0);
    commands.spawn(cam);

    
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Score: 0",
                TextStyle {
                    font: asset_server.load("fonts/atos:.ttf"),
                    font_size: 24.0,
                    color: Color::srgb(1.0, 1.0, 1.0),
                },
            ),
            TextSection::new(
                "    ",
                TextStyle {
                    font: asset_server.load("fonts/atos.ttf"),
                    font_size: 24.0,
                    color: Color::srgb(1.0, 1.0, 1.0),
                },
            ),
            TextSection::new(
                "Mode: Normal",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 24.0,
                    color: Color::srgb(1.0, 1.0, 1.0),
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
        HudText,
    ));
    
    let mut walls = HashSet::new();
    let mut pellets_total = 0usize;

    for y in 0..H {
        for x in 0..W {
            let g = GridPos { x: x as i32, y: (H - 1 - y) as i32 };
            let world = g.to_world();

            match MAP[y][x] {
                0 => {
    
                    walls.insert(g);
                    commands.spawn((
                        SpriteBundle {
                            sprite: Sprite {
                                color: Color::srgb(0.15, 0.25, 0.9),
                                custom_size: Some(Vec2::splat(TILE_SIZE - 2.0)),
                                ..default()
                            },
                            transform: Transform::from_translation(world),
                            ..default()
                        },
                        Solid,
                        GridTransform { pos: g },
                    ));
                }
                1 => {
                    commands.spawn(SpriteBundle {
                        sprite: Sprite {
                            color: Color::srgb(0.0, 0.0, 0.0),
                            custom_size: Some(Vec2::splat(TILE_SIZE - 2.0)),
                            ..default()
                        },
                        transform: Transform::from_translation(world),
                        ..default()
                    });

                    commands.spawn((
                        SpriteBundle {
                            sprite: Sprite {
                                color: Color::srgb_u8(255, 255, 0), 
                                custom_size: Some(Vec2::splat(TILE_SIZE * 0.25)),
                                ..default()
                            },
                            transform: Transform::from_translation(world + Vec3::new(0.0, 0.0, 1.0)),
                            ..default()
                        },
                        Pellet,
                        GridTransform { pos: g },
                    ));
                    pellets_total += 1;
                }
                2 => {
                    commands.spawn(SpriteBundle {
                        sprite: Sprite {
                            color: Color::srgb(0.0, 0.0, 0.0),
                            custom_size: Some(Vec2::splat(TILE_SIZE - 2.0)),
                            ..default()
                        },
                        transform: Transform::from_translation(world),
                        ..default()
                    });

                    commands.spawn((
                        SpriteBundle {
                            sprite: Sprite {
                                color: Color::srgb_u8(255, 69, 0), 
                                custom_size: Some(Vec2::splat(TILE_SIZE * 0.5)),
                                ..default()
                            },
                            transform: Transform::from_translation(world + Vec3::new(0.0, 0.0, 1.0)),
                            ..default()
                        },
                        PowerPellet,
                        GridTransform { pos: g },
                    ));
                    pellets_total += 1;
                }
                9 => {
                    commands.spawn(SpriteBundle {
                        sprite: Sprite {
                            color: Color::srgb(0.0, 0.0, 0.0),
                            custom_size: Some(Vec2::splat(TILE_SIZE - 2.0)),
                            ..default()
                        },
                        transform: Transform::from_translation(world),
                        ..default()
                    });
                }
                _ => {}
            }
        }
    }

    wallset.0 = walls;
    pellet_count.total = pellets_total;
    pellet_count.remaining = pellets_total;
}

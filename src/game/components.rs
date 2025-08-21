use bevy::prelude::*;
use std::collections::HashSet;

pub const TILE_SIZE: f32 = 32.0;
pub const MAP_WIDTH: usize = 19;
pub const MAP_HEIGHT: usize = 21;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GridPos {
    pub x: i32,
    pub y: i32,
}

impl GridPos {
    pub fn to_world(self) -> Vec3 {
        
        let origin_x = -(MAP_WIDTH as f32) * TILE_SIZE * 0.5 + TILE_SIZE * 0.5;
        let origin_y = -(MAP_HEIGHT as f32) * TILE_SIZE * 0.5 + TILE_SIZE * 0.5;
        Vec3::new(
            origin_x + self.x as f32 * TILE_SIZE,
            origin_y + self.y as f32 * TILE_SIZE,
            0.0,
        )
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Ghost;

#[derive(Component)]
pub struct Solid; 

#[derive(Component)]
pub struct Pellet;

#[derive(Component)]
pub struct PowerPellet;

#[derive(Component)]
pub struct GridTransform {
    pub pos: GridPos,
}

#[derive(Resource, Default)]
pub struct WallSet(pub HashSet<GridPos>);

#[derive(Resource, Default)]
pub struct PelletCount {
    pub total: usize,
    pub remaining: usize,
}

#[derive(Resource, Default)]
pub struct Score(pub u32);

#[derive(Resource)]
pub struct PowerMode {
    pub active: bool,
    pub timer: Timer,
}
impl Default for PowerMode {
    fn default() -> Self {
        Self {
            active: false,
            timer: Timer::from_seconds(6.0, TimerMode::Once),
        }
    }
}

#[derive(Resource)]
pub struct GhostStepTimer(pub Timer); 
impl Default for GhostStepTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.18, TimerMode::Repeating))
    }
}

#[derive(Resource)]
pub struct PlayerStepTimer(pub Timer); 
impl Default for PlayerStepTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.11, TimerMode::Repeating))
    }
}

pub fn in_bounds(p: GridPos) -> bool {
    p.x >= 0 && p.y >= 0 && p.x < MAP_WIDTH as i32 && p.y < MAP_HEIGHT as i32
}

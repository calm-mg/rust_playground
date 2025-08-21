use bevy::prelude::*;

use super::{
    components::*,
    ghost::{resolve_collisions, spawn_ghosts, step_ghosts},
    map::setup_world,
    player::{eat_pellets, read_input, spawn_player, step_player, tick_power_mode, InputDir},
    ui::update_hud,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<WallSet>()
            .init_resource::<PelletCount>()
            .init_resource::<Score>()
            .init_resource::<PowerMode>()
            .init_resource::<GhostStepTimer>()
            .init_resource::<PlayerStepTimer>()
            .init_resource::<InputDir>()
            .add_systems(Startup, (setup_world, spawn_player, spawn_ghosts))
            .add_systems(Update, (read_input, update_hud, tick_power_mode))
            .add_systems(FixedUpdate, (step_player, eat_pellets, step_ghosts, resolve_collisions));
    }
}

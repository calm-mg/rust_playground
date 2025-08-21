use bevy::prelude::*;
use crate::game::components::*;

#[derive(Resource, Default)]
pub struct InputDir(pub IVec2);

pub fn spawn_player(mut commands: Commands) {
    let start = GridPos { x: 9, y: 10 }; 
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(1.0, 1.0, 0.2),
                custom_size: Some(Vec2::splat(TILE_SIZE - 6.0)),
                ..default()
            },
            transform: Transform::from_translation(start.to_world() + Vec3::new(0.0, 0.0, 2.0)),
            ..default()
        },
        Player,
        GridTransform { pos: start },
    ));
}

pub fn read_input(mut input_dir: ResMut<InputDir>, kb: Res<ButtonInput<KeyCode>>) {
    let mut dir = IVec2::ZERO;
    if kb.pressed(KeyCode::ArrowUp)    || kb.pressed(KeyCode::KeyW) { dir = IVec2::new(0,  1); }
    if kb.pressed(KeyCode::ArrowDown)  || kb.pressed(KeyCode::KeyS) { dir = IVec2::new(0, -1); }
    if kb.pressed(KeyCode::ArrowLeft)  || kb.pressed(KeyCode::KeyA) { dir = IVec2::new(-1, 0); }
    if kb.pressed(KeyCode::ArrowRight) || kb.pressed(KeyCode::KeyD) { dir = IVec2::new(1,  0); }
    if dir != IVec2::ZERO {
        input_dir.0 = dir;
    }
}

pub fn step_player(
    time: Res<Time>,
    mut pstep: ResMut<PlayerStepTimer>,
    walls: Res<WallSet>,
    input: Res<InputDir>,
    mut q: Query<(&mut GridTransform, &mut Transform), With<Player>>,
) {
    if !pstep.0.tick(time.delta()).just_finished() {
        return;
    }
    if let Ok((mut grid, mut tf)) = q.get_single_mut() {
        let n = GridPos { x: grid.pos.x + input.0.x, y: grid.pos.y + input.0.y };
        if in_bounds(n) && !walls.0.contains(&n) {
            grid.pos = n;
            tf.translation = grid.pos.to_world() + Vec3::new(0.0, 0.0, 2.0);
        }
    }
}

pub fn eat_pellets(
    mut commands: Commands,
    mut pellet_count: ResMut<PelletCount>,
    mut score: ResMut<Score>,
    mut power: ResMut<PowerMode>,
    q_player: Query<&GridTransform, With<Player>>,
    q_pellet: Query<(Entity, &GridTransform), With<Pellet>>,
    q_power: Query<(Entity, &GridTransform), With<PowerPellet>>,
) {
    let Ok(pgt) = q_player.get_single() else { return; };

    for (e, gt) in q_pellet.iter() {
        if gt.pos == pgt.pos {
            commands.entity(e).despawn();
            pellet_count.remaining = pellet_count.remaining.saturating_sub(1);
            score.0 += 10;
            break;
        }
    }
    for (e, gt) in q_power.iter() {
        if gt.pos == pgt.pos {
            commands.entity(e).despawn();
            pellet_count.remaining = pellet_count.remaining.saturating_sub(1);
            score.0 += 50;
            power.active = true;
            power.timer.reset();
            break;
        }
    }
}

pub fn tick_power_mode(time: Res<Time>, mut power: ResMut<PowerMode>) {
    if power.active && power.timer.tick(time.delta()).finished() {
        power.active = false;
    }
}

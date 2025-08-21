use bevy::prelude::*;
use crate::game::components::*;

pub fn spawn_ghosts(mut commands: Commands) {
    let starts = [
        GridPos { x: 9, y: 13 },
        GridPos { x: 8, y: 13 },
        GridPos { x:10, y: 13 },
        GridPos { x: 9, y: 12 },
    ];
    for s in starts {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(1.0, 0.2, 0.2),
                    custom_size: Some(Vec2::splat(TILE_SIZE - 6.0)),
                    ..default()
                },
                transform: Transform::from_translation(s.to_world() + Vec3::new(0.0, 0.0, 2.0)),
                ..default()
            },
            Ghost,
            GridTransform { pos: s },
        ));
    }
}

fn neighbors(p: GridPos) -> [GridPos; 4] {
    [
        GridPos { x: p.x + 1, y: p.y },
        GridPos { x: p.x - 1, y: p.y },
        GridPos { x: p.x, y: p.y + 1 },
        GridPos { x: p.x, y: p.y - 1 },
    ]
}

pub fn step_ghosts(
    time: Res<Time>,
    mut gstep: ResMut<GhostStepTimer>,
    walls: Res<WallSet>,
    power: Res<PowerMode>,
    
    mut q_ghosts: Query<
        (&mut GridTransform, &mut Transform),
        (With<Ghost>, Without<Player>)
    >,
    q_player: Query<
        &GridTransform,
        (With<Player>, Without<Ghost>)
    >,
) {
    if !gstep.0.tick(time.delta()).just_finished() {
        return;
    }
    let Ok(pgt) = q_player.get_single() else { return; };
    for (mut ggt, mut tf) in q_ghosts.iter_mut() {
        
        let mut best = ggt.pos;
        let mut best_score = i32::MIN;
        for n in neighbors(ggt.pos) {
            if !in_bounds(n) || walls.0.contains(&n) { continue; }
            let d = (pgt.pos.x - n.x).abs() + (pgt.pos.y - n.y).abs();
            let score = if power.active { d } else { -d };
            if score > best_score {
                best_score = score;
                best = n;
            }
        }
        ggt.pos = best;
        tf.translation = ggt.pos.to_world() + Vec3::new(0.0, 0.0, 2.0);
    }
}

pub fn resolve_collisions(
    mut commands: Commands,
    mut score: ResMut<Score>,
    power: Res<PowerMode>,
    q_player: Query<&GridTransform, (With<Player>, Without<Ghost>)>,
    q_ghosts: Query<(Entity, &GridTransform), (With<Ghost>, Without<Player>)>,
) {
    let Ok(pgt) = q_player.get_single() else { return; };
    for (e, ggt) in q_ghosts.iter() {
        if ggt.pos == pgt.pos {
            if power.active {
                commands.entity(e).despawn();
                score.0 += 200;
            } else {
            }
        }
    }
}

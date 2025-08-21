use bevy::prelude::*;
use crate::game::components::*;

#[derive(Component)]
pub struct HudText;

pub fn update_hud(
    score: Res<Score>,
    power: Res<PowerMode>,
    mut q: Query<&mut Text, With<HudText>>,
) {
    if let Ok(mut text) = q.get_single_mut() {
        text.sections[0].value = format!("Score: {}", score.0);
        text.sections[2].value = if power.active { "Mode: POWER" } else { "Mode: Normal" }.into();
        text.sections[2].style.color =
            if power.active { Color::srgb_u8(255, 69, 0) } else { Color::srgb(1.0, 1.0, 1.0) };
    }
}

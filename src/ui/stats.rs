use crate::controller::LocalController;
use crate::settings::GameSettings;
use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use std::collections::BTreeMap;
use std::time::Duration;

#[derive(Component)]
pub struct StatsText;

pub fn setup_stats(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_sections([TextSection::from_style(TextStyle {
            font: asset_server.load("typefaces/monogram-extended.ttf"),
            font_size: 30.0,
            color: Color::WHITE,
        })])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(0.0),
                right: Val::Px(7.0),
                ..Default::default()
            },
            // align_content: AlignContent::FlexEnd,
            // align_items: AlignItems::FlexEnd,
            ..Default::default()
        }),
        StatsText,
    ));
}

pub fn update_stats(
    time: Res<Time>,
    settings: Res<GameSettings>,
    diagnostics: Res<Diagnostics>,
    local_player_camera: Query<&Transform, With<LocalController>>,
    mut query: Query<(&mut Text, &mut Visibility), With<StatsText>>,
    mut samples: Local<BTreeMap<Duration, f64>>,
) {
    let (mut text, mut visibility) = query.single_mut();

    if settings.show_stats {
        *visibility = Visibility::Visible;
    } else {
        *visibility = Visibility::Hidden;
        return;
    }

    let mut s = Vec::new();

    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(value) = fps.value() {
            // Update the value of the second section
            s.push(format!("FPS: {value:.2}"));

            samples.insert(time.elapsed(), value);

            let duration = Duration::from_secs(1);
            if time.elapsed() > duration {
                let oldest = time.elapsed() - duration;
                *samples = samples.split_off(&oldest);

                // Average the samples and print it as AVG:
                let avg = samples.values().sum::<f64>() / samples.len() as f64;
                s.push(format!("AVG: {avg:.2}"));
            }
        }
    }

    // Show position as x,y,z and orientation as pitch,yaw,roll
    if let Ok(transform) = local_player_camera.get_single() {
        s.push(format!(
            "POS: {:.1},{:.1},{:.1}",
            transform.translation.x, transform.translation.y, transform.translation.z
        ));
        let q = transform.rotation;
        let pitch = q.x.atan2((q.w * q.w + q.z * q.z).sqrt());
        let yaw = q.y.atan2((q.w * q.w + q.x * q.x).sqrt());
        let roll = q.z.atan2((q.w * q.w + q.y * q.y).sqrt());

        let pitch = pitch.to_degrees();
        let yaw = yaw.to_degrees();
        let roll = roll.to_degrees();

        s.push(format!("ROT: {:.0},{:.0},{:.0}", pitch, yaw, roll));
    }

    text.sections[0].value = s.join("\n");
}

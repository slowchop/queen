use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy::render::primitives::Frustum;
use bevy::window::{PresentMode, PrimaryWindow};
use bevy::winit;
use bevy::winit::WinitWindows;
use std::slice::Windows;
use std::thread::current;

pub fn setup(mut commands: Commands, mut windows: Query<&mut Window, With<PrimaryWindow>>) {
    // commands.spawn(Camera2dBundle {
    //     camera_2d: Camera2d {
    //         clear_color: ClearColorConfig::Custom(Color::INDIGO),
    //     },
    //     ..Default::default()
    // });

    let mut window = windows.single_mut();
    window.present_mode = PresentMode::AutoNoVsync;
}

// TODO: This builds but we're not using it yet.
// pub fn enumerate_resolutions(winit_windows: NonSendMut<WinitWindows>) {
//     for monitor in winit_windows
//         .windows
//         .values()
//         .next()
//         .unwrap()
//         .available_monitors()
//     {
//         println!("{:?}", monitor.video_modes().collect::<Vec<_>>());
//     }
// }

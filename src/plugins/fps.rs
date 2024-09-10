#![allow(non_snake_case)]

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Resource)]
struct FPS {
    buffer: [f32; 10],
    index: i64,
}

pub struct FpsPlugin;
impl Plugin for FpsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, fps)
           .insert_resource(FPS {
                buffer: [0.0; 10],
                index: 0,
            });
    }
}

fn fps(
    time: Res<Time>,
    mut FPS: ResMut<FPS>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    FPS.index += 1;
    let i = FPS.index;
    let fps = 1. / time.delta_seconds();

    FPS.buffer[(i % 10) as usize] = fps;

    if i % 10 != 0 {
        return;
    }
    if let Ok(mut window) = window_query.get_single_mut() {
        let mut sum = 0.;
        for n in 0..10 {
            sum += FPS.buffer[n];
        }
        window.title = format!("FPS:{:0.2}", sum / 10.);
    }
}

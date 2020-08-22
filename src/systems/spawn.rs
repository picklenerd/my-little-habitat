use bevy::{
    input::mouse::MouseButtonInput,
    prelude::*,
    window::CursorMoved,
};
use serde_json::json;

use crate::{
    creatures::{Energy, Position, Plant},
};


#[derive(Default)]
pub struct MouseState {
    pub position: Position,
    pub left: bool,
    pub right: bool,
    mouse_button_event_reader: EventReader<MouseButtonInput>,
    cursor_moved_event_reader: EventReader<CursorMoved>,
}

impl MouseState {
    fn update(&mut self,
        mouse_button_input_events: &Res<Events<MouseButtonInput>>, 
        cursor_moved_events: &Res<Events<CursorMoved>>,
        window: &Res<WindowDescriptor>,
    ) {
        for event in self.mouse_button_event_reader.iter(&mouse_button_input_events) {
            use bevy::input::keyboard::ElementState;
    
            match event.button {
                MouseButton::Left => self.left = event.state == ElementState::Pressed,
                MouseButton::Right => self.right = event.state == ElementState::Pressed,
                _ => (),
            }
        }
    
        for event in self.cursor_moved_event_reader.iter(&cursor_moved_events) {
            let width = window.width as f32;
            let height = window.height as f32;

            let x = event.position.x() / width;
            let y = event.position.y() / height;

            let x = (2.0 * x) - 1.0;
            let y = (2.0 * y) - 1.0;

            self.position = Position { x, y };
        }
    }
}

pub fn spawn(
    mut commands: Commands,
    mut state: ResMut<MouseState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    window: Res<WindowDescriptor>,
    mouse_button_input_events: Res<Events<MouseButtonInput>>,
    cursor_moved_events: Res<Events<CursorMoved>>,
) {
    state.update(&mouse_button_input_events, &cursor_moved_events, &window);
   
    if state.left {
        let Position { x, y } = state.position;

        // println!("{}", json!({
        //     "state": {
        //         "x": state.position.x,
        //         "y": state.position.y,
        //     },
        //     "calc": {
        //         "x": x,
        //         "y": y,
        //     },
        //     "window": {
        //         "width": window.width,
        //         "height": window.height,
        //     }
        // }));

        commands.spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Quad { size: (10.0, 10.0).into(), flip: false })),
            material: materials.add(Color::rgb(0.0, 0.8, 0.2).into()),
            ..Default::default()
        })
            .with(Plant)
            .with(Energy(100))
            .with(Position { x, y });
    }
}

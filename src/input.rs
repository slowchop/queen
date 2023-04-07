use crate::state;
use crate::state::MenuState;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::prelude::*;
use bevy::utils::HashMap;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

#[derive(Resource, Deref, DerefMut)]
pub struct KeyboardInputMap(HashMap<KeyCode, InputAction>);

#[derive(Resource, Deref, DerefMut)]
pub struct MouseButtonInputMap(HashMap<MouseButton, InputAction>);

#[derive(Resource, Deref, DerefMut)]
pub struct InputStates(HashMap<InputAction, InputState>);

impl InputStates {
    pub fn is_pressed(&self, action: InputAction) -> bool {
        self.0
            .get(&action)
            .map(|state| state.is_pressed)
            .unwrap_or(false)
    }

    pub fn just_pressed(&self, action: InputAction) -> bool {
        self.0
            .get(&action)
            .map(|state| state.just_pressed)
            .unwrap_or(false)
    }
}

#[derive(Default, Copy, Clone)]
pub struct InputState {
    pub just_pressed: bool,
    pub just_released: bool,
    pub is_pressed: bool,
}

#[derive(Eq, PartialEq)]
pub enum EventState {
    Pressed,
    Released,
}

pub struct ActionEvent {
    pub action: InputAction,
    pub state: EventState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, strum::EnumIter)]
pub enum InputAction {
    Up,
    Down,
    Left,
    Right,
    PrimaryAction,
    SecondaryAction,
    Confirm,
    Escape,
    TogglePause,
    Speed1,
    Speed2,
    Speed3,
    Debug1,
    Debug2,
}

pub fn setup(mut commands: Commands) {
    let mut keyboard_input_map = HashMap::new();
    let mut mouse_button_input_map = HashMap::new();

    keyboard_input_map.insert(KeyCode::Up, InputAction::Up);
    keyboard_input_map.insert(KeyCode::Down, InputAction::Down);
    keyboard_input_map.insert(KeyCode::Left, InputAction::Left);
    keyboard_input_map.insert(KeyCode::Right, InputAction::Right);

    keyboard_input_map.insert(KeyCode::W, InputAction::Up);
    keyboard_input_map.insert(KeyCode::S, InputAction::Down);
    keyboard_input_map.insert(KeyCode::A, InputAction::Left);
    keyboard_input_map.insert(KeyCode::D, InputAction::Right);

    keyboard_input_map.insert(KeyCode::Return, InputAction::Confirm);

    keyboard_input_map.insert(KeyCode::Space, InputAction::TogglePause);
    keyboard_input_map.insert(KeyCode::Key1, InputAction::Speed1);
    keyboard_input_map.insert(KeyCode::Key2, InputAction::Speed2);
    keyboard_input_map.insert(KeyCode::Key3, InputAction::Speed3);

    keyboard_input_map.insert(KeyCode::F1, InputAction::Debug1);
    keyboard_input_map.insert(KeyCode::F2, InputAction::Debug2);

    mouse_button_input_map.insert(MouseButton::Left, InputAction::PrimaryAction);
    mouse_button_input_map.insert(MouseButton::Right, InputAction::SecondaryAction);

    commands.insert_resource(KeyboardInputMap(keyboard_input_map));
    commands.insert_resource(MouseButtonInputMap(mouse_button_input_map));

    let mut input_states = HashMap::new();
    for action in InputAction::iter() {
        input_states.insert(action, InputState::default());
    }
    commands.insert_resource(InputStates(input_states));

    // Add Event type to world
    commands.insert_resource(Events::<ActionEvent>::default());
}

pub fn reset_input_states(mut input_states: ResMut<InputStates>) {
    // Set all "just_pressed" and "just_released" to false.
    for state in input_states.values_mut() {
        state.just_pressed = false;
        state.just_released = false;
    }
}

pub fn process_keyboard_input(
    mut input_states: ResMut<InputStates>,
    keyboard_input_map: Res<KeyboardInputMap>,
    mut keyboard_events: EventReader<KeyboardInput>,
    mut input_action_writer: EventWriter<ActionEvent>,
) {
    for event in keyboard_events.iter() {
        let Some(key_code) = event.key_code else {
            continue;
        };
        let Some(action) = keyboard_input_map.get(&key_code) else {
            continue;
        };

        let Some(existing_state) = input_states.get_mut(action) else {
            warn!("No input state for action: {:?}", action);
            continue;
        };

        let event_state = match event.state {
            ButtonState::Pressed => {
                if existing_state.is_pressed {
                    continue;
                }

                existing_state.is_pressed = true;
                existing_state.just_pressed = true;
                EventState::Pressed
            }
            ButtonState::Released => {
                if !existing_state.is_pressed {
                    continue;
                }

                existing_state.is_pressed = false;
                existing_state.just_released = true;
                EventState::Released
            }
        };

        input_action_writer.send(ActionEvent {
            action: *action,
            state: event_state,
        });

        // info!("Keyboard event: {:?}", event);
    }
}

pub fn process_mouse_input(
    mut input_states: ResMut<InputStates>,
    mouse_button_input_map: Res<MouseButtonInputMap>,
    mut mouse_button_events: EventReader<MouseButtonInput>,
    mut input_action_writer: EventWriter<ActionEvent>,
) {
    for event in mouse_button_events.iter() {
        let Some(action) = mouse_button_input_map.get(&event.button) else {
            continue;
        };
        let (input_state, event_state) = match event.state {
            ButtonState::Pressed => (
                InputState {
                    is_pressed: true,
                    just_pressed: true,
                    just_released: false,
                },
                EventState::Pressed,
            ),
            ButtonState::Released => (
                InputState {
                    is_pressed: false,
                    just_pressed: false,
                    just_released: true,
                },
                EventState::Released,
            ),
        };

        input_states.insert(*action, input_state);

        input_action_writer.send(ActionEvent {
            action: *action,
            state: event_state,
        });

        // info!("Mouse event: {:?}", event);
    }
}

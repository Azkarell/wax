pub mod actions;
pub mod input;

use std::{env, path::PathBuf, str::FromStr};

use anathema::{
    component::{Component, KeyEvent, KeyState},
    state::{List, State, Value},
};

use crate::{
    action_map::{ActionKey, ActionMap},
    components::actions::{down, get_entries_as_string, select, up},
};

pub struct ModelPicker(pub ActionMap<ModelPickerState>);

impl Default for ModelPicker {
    fn default() -> Self {
        let mut default_map = ActionMap::new();
        default_map.add_action(up, ActionKey('k', false), KeyState::Press);
        default_map.add_action(down, ActionKey('j', false), KeyState::Press);
        default_map.add_action(select, ActionKey(' ', false), KeyState::Press);
        Self(default_map)
    }
}

impl Component for ModelPicker {
    type State = ModelPickerState;

    type Message = ();

    fn on_key(
        &mut self,
        key: anathema::component::KeyEvent,
        state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        mut _context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        self.0.handle_event(state, key);
    }
}

#[derive(State, Debug)]
pub struct ModelPickerState {
    cwd: Value<String>,
    entries: Value<List<String>>,
    current: Value<usize>,
}

impl ModelPickerState {
    pub fn new() -> std::io::Result<Self> {
        let cwd = env::current_dir()?;
        let s = Self {
            cwd: Value::new(cwd.to_string_lossy().into_owned()),
            entries: Value::new(List::from_iter(get_entries_as_string(cwd)?)),
            current: 0.into(),
        };
        Ok(s)
    }
}

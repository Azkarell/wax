use anathema::{
    component::Component,
    state::{State, Value},
};

pub struct Input;

impl Input {
    pub fn new() -> Self {
        Self
    }
}

#[derive(State)]
pub struct InputState {
    value: Value<String>,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            value: Value::new(String::new()),
        }
    }
}

impl Component for Input {
    type State = InputState;

    type Message = ();

    fn on_mount(
        &mut self,
        state: &mut Self::State,
        mut children: anathema::component::Children<'_, '_>,
        mut context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        if let Some(val) = context.attribute("value") {
            match val {
                anathema::resolver::ValueKind::Int(v) => state.value.set(format!("{v}")),
                anathema::resolver::ValueKind::Float(v) => state.value.set(format!("{v}")),
                anathema::resolver::ValueKind::Bool(v) => state.value.set(format!("{v}")),
                anathema::resolver::ValueKind::Char(v) => state.value.set(format!("{v}")),
                anathema::resolver::ValueKind::Hex(hex) => state.value.set(format!("{hex}")),
                anathema::resolver::ValueKind::Color(color) => state.value.set(format!("{color}")),
                anathema::resolver::ValueKind::Str(cow) => state.value.set(format!("{cow}")),
                _ => {}
            }
        }
    }

    fn on_key(
        &mut self,
        key: anathema::component::KeyEvent,
        state: &mut Self::State,
        mut children: anathema::component::Children<'_, '_>,
        mut context: anathema::component::Context<'_, '_, Self::State>,
    ) {
    }
}

mod action_map;
mod components;

use anathema::{
    component::{self, Event, KeyCode, KeyEvent, KeyState},
    prelude::{Document, TuiBackend},
    widgets::{components::deferred::DeferredComponents, tabindex::TabIndex},
};
use components::input::{Input, InputState};

use crate::components::{ModelPicker, ModelPickerState};

fn quit_event_handler(
    ev: Event,
    _tab_index: &mut TabIndex,
    _components: &mut DeferredComponents,
) -> Option<Event> {
    match ev {
        Event::Key(KeyEvent {
            code: KeyCode::Char('q'),
            ctrl: false,
            state: KeyState::Press,
        }) => Some(Event::Stop),
        _ => Some(ev),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let device = Device::Cpu;
    let document = Document::new("@index");
    let mut backend = TuiBackend::builder()
        .enable_alt_screen()
        .enable_raw_mode()
        .hide_cursor()
        .clear()
        .finish()
        .unwrap();

    let mut builder = anathema::prelude::Runtime::builder(document, &backend)
        .with_global_event_handler(quit_event_handler);

    // let mut container = get_gguf_container("");
    builder
        .default::<()>("index", "templates/index.aml")
        .unwrap();
    builder
        .component(
            "model_picker",
            "templates/model_picker.aml",
            ModelPicker::default(),
            ModelPickerState::new()?,
        )
        .unwrap();
    builder.prototype("inptu", "templates/input.aml", Input::new, InputState::new)
    builder
        .finish(&mut backend, |mut rt, backend| rt.run(backend))
        .unwrap();
    Ok(())
}

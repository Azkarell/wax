pub mod action_map;
pub mod components;
use crate::anathema_app::components::{
    ModelPicker, ModelPickerState,
    input::{Input, InputState},
};
use anathema::{
    component::{Event, KeyCode, KeyEvent, KeyState},
    prelude::{Document, TuiBackend},
    widgets::{components::deferred::DeferredComponents, tabindex::TabIndex},
};

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
fn start_anathema() -> Result<(), Box<dyn std::error::Error>> {
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

    builder.prototype("input", "templates/input.aml", Input::new, InputState::new)?;
    builder
        .finish(&mut backend, |rt, backend| rt.run(backend))
        .unwrap();
    Ok(())
}

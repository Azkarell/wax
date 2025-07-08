use std::{path::PathBuf, str::FromStr};

use anathema::state::List;

use crate::anathema_app::components::ModelPickerState;

fn decrement_wrapped(current: &mut usize, max_len: usize) {
    *current += 1;
    if *current >= max_len {
        *current = 0;
    }
}

fn increment_wrapped(current: &mut usize, max_len: usize) {
    if *current == 0 {
        *current = max_len - 1;
    } else {
        *current -= 1;
    }
}

pub fn up(state: &mut ModelPickerState) {
    let mut current = state.current.to_mut();
    let max = state.entries.len();
    decrement_wrapped(&mut current, max);
}
pub fn down(state: &mut ModelPickerState) {
    let mut current = state.current.to_mut();
    let max = state.entries.len();
    increment_wrapped(&mut current, max);
}

pub fn select(state: &mut ModelPickerState) {
    let val = state
        .entries
        .to_ref()
        .get(state.current.copy_value())
        .unwrap()
        .to_ref()
        .clone();

    let current_path = PathBuf::from_str(&state.cwd.to_ref())
        .unwrap()
        .join(PathBuf::from_str(&val).unwrap());

    let new_entries = get_entries_as_string(current_path.clone()).unwrap();

    *state.cwd.to_mut() = current_path.to_string_lossy().into_owned();
    state.entries.set(List::from_iter(new_entries));
}
pub fn get_entries_as_string(cwd: PathBuf) -> std::io::Result<impl Iterator<Item = String>> {
    let dir = std::fs::read_dir(cwd.clone())?;
    let iter = dir.filter_map(move |d| {
        let Ok(entry) = d else {
            return None;
        };
        Some(
            entry
                .path()
                .strip_prefix(&cwd)
                .unwrap()
                .to_string_lossy()
                .into_owned(),
        )
    });
    Ok(iter)
}

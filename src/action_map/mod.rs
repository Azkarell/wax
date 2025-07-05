use std::{cell::RefCell, collections::HashMap, marker::PhantomData};

use anathema::{
    component::{KeyCode, KeyEvent, KeyState},
    state::State,
};

pub trait Action<S: State>: std::fmt::Debug {
    fn run(&self, state: &mut S);
}

#[derive(PartialEq, Debug, Eq, Hash)]
pub struct ActionKey(pub char, pub bool);

#[derive(Debug)]
pub struct ActionMap<S: State> {
    on_press: HashMap<ActionKey, Box<dyn Action<S>>>,
    on_release: HashMap<ActionKey, Box<dyn Action<S>>>,
    on_repeat: HashMap<ActionKey, Box<dyn Action<S>>>,
}

pub trait IntoBoxedAction<S> {
    fn into_boxed_action(self) -> Box<dyn Action<S>>;
}
pub struct FunctionAction<S: State, F: FnMut(&mut S)> {
    f: RefCell<F>,
    _pd: PhantomData<S>,
}

impl<S: State, F: FnMut(&mut S)> std::fmt::Debug for FunctionAction<S, F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FunctionAction").finish()
    }
}

impl<S: State, F: FnMut(&mut S)> Action<S> for FunctionAction<S, F> {
    fn run(&self, state: &mut S) {
        (self.f.borrow_mut())(state)
    }
}

impl<S: State, F: FnMut(&mut S) + 'static> IntoBoxedAction<S> for F {
    fn into_boxed_action(self) -> Box<dyn Action<S>> {
        Box::new(FunctionAction {
            f: RefCell::new(self),
            _pd: PhantomData,
        })
    }
}
impl<S: State> ActionMap<S> {
    pub fn new() -> Self {
        Self {
            on_release: HashMap::new(),
            on_press: HashMap::new(),
            on_repeat: HashMap::new(),
        }
    }

    pub fn add_action<A: IntoBoxedAction<S>>(
        &mut self,
        action: A,
        key: ActionKey,
        state: KeyState,
    ) {
        match state {
            KeyState::Press => self.on_press.insert(key, action.into_boxed_action()),
            KeyState::Repeat => self.on_repeat.insert(key, action.into_boxed_action()),
            KeyState::Release => self.on_release.insert(key, action.into_boxed_action()),
        };
    }
    pub fn handle_event(&self, s: &mut S, key_event: KeyEvent) {
        if let KeyEvent {
            code: KeyCode::Char(c),
            ctrl,
            state,
        } = key_event
        {
            match state {
                KeyState::Press => {
                    if let Some(a) = self.on_press.get(&ActionKey(c, ctrl)) {
                        a.run(s);
                    }
                }

                KeyState::Repeat => {
                    if let Some(a) = self.on_repeat.get(&ActionKey(c, ctrl)) {
                        a.run(s);
                    }
                }
                KeyState::Release => {
                    if let Some(a) = self.on_release.get(&ActionKey(c, ctrl)) {
                        a.run(s);
                    }
                }
            }
        }
    }
}

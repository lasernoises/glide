use crossterm::event::KeyEvent;
use ratatui::prelude::{Buffer, Position, Rect};

use crate::{
    reactivity::{Changeable, ReactivityNodes},
    widget::{self, Focus, Widget, WidgetState},
};

pub fn state<Out, T: Changeable + 'static, W: Widget<StateOut<T::Change, Out>>>(
    init: impl Fn() -> T,
    content: impl Fn(T) -> W,
) -> impl Widget<Out> {
    State { init, content }
}

pub fn change<C, O: Default>(change: C) -> StateOut<C, O> {
    StateOut {
        change,
        outer: Default::default(),
    }
}

pub struct StateOut<C, O> {
    change: C,
    outer: O,
}

struct State<I, C> {
    init: I,
    content: C,
}

struct StateState<T: Changeable, S> {
    value: T,
    reactivity: T::Reactivity,

    // last_change: T::Change,
    inner: S,
}

impl<Out, T: Changeable, S: WidgetState<StateOut<T::Change, Out>>> WidgetState<Out>
    for StateState<T, S>
{
    fn reset_focus(&mut self) -> widget::Focusable {
        self.inner.reset_focus()
    }

    fn draw(&self, focus: Focus, area: Rect, buffer: &mut Buffer) -> Option<Position> {
        self.inner.draw(focus, area, buffer)
    }

    fn handle_key_event(
        &mut self,
        reactivity_nodes: &mut ReactivityNodes,
        event: KeyEvent,
    ) -> Option<Out> {
        if let Some(out) = self.inner.handle_key_event(reactivity_nodes, event) {
            // TODO
            self.value = self
                .value
                .apply(&mut self.reactivity, reactivity_nodes, out.change);

            Some(out.outer)
        } else {
            None
        }
    }
}

impl<Out, T: Changeable, W: Widget<StateOut<T::Change, Out>>, I: Fn() -> T, C: Fn(T) -> W>
    Widget<Out> for State<I, C>
{
    type State = StateState<T, W::State>;

    fn init(&self, reactivity_nodes: &mut ReactivityNodes) -> Self::State {
        let value = (self.init)();

        StateState {
            value,
            reactivity: value.init_reactivity(reactivity_nodes),
            inner: (self.content)(value).init(reactivity_nodes),
        }
    }

    fn update(&self, state: &mut Self::State) {
        (self.content)(state.value).update(&mut state.inner);
    }
}

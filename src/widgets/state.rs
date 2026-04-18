use crossterm::event::KeyEvent;
use ratatui::prelude::{Buffer, Position, Rect};

use crate::{
    reactivity::Changeable,
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

struct StateState<T, S> {
    value: T,
    // last_change: T::Change,
    inner: S,
}

impl<T, S: WidgetState> WidgetState for StateState<T, S> {
    fn reset_focus(&mut self) -> widget::Focusable {
        self.inner.reset_focus()
    }

    fn draw(&self, focus: Focus, area: Rect, buffer: &mut Buffer) -> Option<Position> {
        self.inner.draw(focus, area, buffer)
    }
}

impl<Out, T: Changeable + 'static, W: Widget<StateOut<T::Change, Out>>, I: Fn() -> T, C: Fn(T) -> W>
    Widget<Out> for State<I, C>
{
    type State = StateState<T, W::State>;

    fn init(&self) -> Self::State {
        let value = (self.init)();

        StateState {
            value,
            inner: (self.content)(value).init(),
        }
    }

    fn handle_key_event(&self, state: &mut Self::State, event: KeyEvent) -> Option<Out> {
        if let Some(out) = (self.content)(state.value).handle_key_event(&mut state.inner, event) {
            // TODO
            state.value = state.value.apply(out.change);

            Some(out.outer)
        } else {
            None
        }
    }

    fn update(&self, state: &mut Self::State) {
        (self.content)(state.value).update(&mut state.inner);
    }
}

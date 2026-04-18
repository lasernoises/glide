use crossterm::event::KeyEvent;
use ratatui::{
    buffer::Buffer,
    layout::{Position, Rect},
    widgets::Widget as _,
};

use crate::reactivity::ReactivityNodes;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Focusable {
    No,
    Yes,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Focus {
    Unfocused,
    Focused,
}

pub trait WidgetState<Out> {
    fn reset_focus(&mut self) -> Focusable;

    fn draw(&self, focus: Focus, area: Rect, buffer: &mut Buffer) -> Option<Position>;

    fn handle_key_event(
        &mut self,
        reactivity_nodes: &mut ReactivityNodes,
        event: KeyEvent,
    ) -> Option<Out>;
}

pub trait Widget<Out> {
    type State: WidgetState<Out>;

    fn init(&self, reactivity_nodes: &mut ReactivityNodes) -> Self::State;

    fn update(&self, state: &mut Self::State);
}

impl<Out> WidgetState<Out> for &'static str {
    fn reset_focus(&mut self) -> Focusable {
        Focusable::No
    }

    fn draw(&self, _focus: Focus, area: Rect, buffer: &mut Buffer) -> Option<Position> {
        self.render(area, buffer);

        None
    }

    fn handle_key_event(&mut self, _: &mut ReactivityNodes, _: KeyEvent) -> Option<Out> {
        None
    }
}

impl<Out> Widget<Out> for &'static str {
    type State = Self;

    fn init(&self, _: &mut ReactivityNodes) -> Self::State {
        self
    }

    fn update(&self, state: &mut Self::State) {
        // TODO: remove. This isn't a reactive widget.
        *state = self;
    }
}

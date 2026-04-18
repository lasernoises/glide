use crossterm::event::KeyEvent;
use ratatui::{
    buffer::Buffer,
    layout::{Position, Rect},
    widgets::Widget as _,
};

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

pub trait WidgetState {
    fn reset_focus(&mut self) -> Focusable;

    fn draw(&self, focus: Focus, area: Rect, buffer: &mut Buffer) -> Option<Position>;
}

pub trait Widget<Out> {
    type State: WidgetState;

    fn init(&self) -> Self::State;

    fn handle_key_event(&self, state: &mut Self::State, event: KeyEvent) -> Option<Out>;
}

impl WidgetState for &'static str {
    fn reset_focus(&mut self) -> Focusable {
        Focusable::No
    }

    fn draw(&self, _focus: Focus, area: Rect, buffer: &mut Buffer) -> Option<Position> {
        self.render(area, buffer);

        None
    }
}

impl<Out> Widget<Out> for &'static str {
    type State = Self;

    fn init(&self) -> Self::State {
        self
    }

    fn handle_key_event(&self, _state: &mut Self::State, _event: KeyEvent) -> Option<Out> {
        None
    }
}

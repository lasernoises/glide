use crossterm::event::KeyEvent;
use ratatui::prelude::{self, Position, Rect};

use crate::widget::{Focus, Focusable, Widget, WidgetState};

pub fn with_key_handler<Out>(
    handler: impl Fn(KeyEvent) -> Option<Out>,
    widget: impl Widget<Out>,
) -> impl Widget<Out> {
    WithKeyHandler { handler, widget }
}

struct WithKeyHandler<H, W> {
    handler: H,
    widget: W,
}

struct State<S>(S);

impl<S: WidgetState> WidgetState for State<S> {
    fn reset_focus(&mut self) -> Focusable {
        Focusable::Yes
    }

    fn draw(&self, focus: Focus, area: Rect, buffer: &mut prelude::Buffer) -> Option<Position> {
        self.0.draw(focus, area, buffer)
    }
}

impl<Out, H: Fn(KeyEvent) -> Option<Out>, W: Widget<Out>> Widget<Out> for WithKeyHandler<H, W> {
    type State = State<W::State>;

    fn init(&self) -> Self::State {
        State(self.widget.init())
    }

    fn handle_key_event(&self, state: &mut Self::State, event: KeyEvent) -> Option<Out> {
        self.widget
            .handle_key_event(&mut state.0, event)
            .or_else(|| (self.handler)(event))
    }

    fn update(&self, state: &mut Self::State) {
        self.widget.update(&mut state.0);
    }
}

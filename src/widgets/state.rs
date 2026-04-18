use ratatui::prelude::{Buffer, Position, Rect};

use crate::widget::{self, Focus, Widget, WidgetState};

pub fn state<Out, T: 'static, W: Widget<Out>>(
    init: impl Fn() -> T,
    content: impl Fn(&'static T) -> W,
) -> impl Widget<Out> {
    State { init, content }
}

struct State<I, C> {
    init: I,
    content: C,
}

struct StateState<T: 'static, S> {
    value: &'static T,
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

impl<Out, T: 'static, W: Widget<Out>, I: Fn() -> T, C: Fn(&'static T) -> W> Widget<Out>
    for State<I, C>
{
    type State = StateState<T, W::State>;

    fn init(&self) -> Self::State {
        // TODO
        let value = Box::leak(Box::new((self.init)()));

        StateState {
            value,
            inner: (self.content)(value).init(),
        }
    }

    fn handle_key_event(
        &self,
        state: &mut Self::State,
        event: crossterm::event::KeyEvent,
    ) -> Option<Out> {
        (self.content)(state.value).handle_key_event(&mut state.inner, event)
    }
}

use crossterm::event::KeyEvent;
use ratatui::prelude::{self, Position, Rect};

use crate::{
    reactivity::{Ctx, ReactivityNodes},
    widget::{Focus, Focusable, Widget, WidgetState},
};

pub fn with_key_handler<Out>(
    handler: impl Copy + Fn(KeyEvent) -> Option<Out>,
    widget: impl Widget<Out>,
) -> impl Widget<Out> {
    WithKeyHandler { handler, widget }
}

struct WithKeyHandler<H, W> {
    handler: H,
    widget: W,
}

struct State<H, S> {
    handler: H,
    inner: S,
}

impl<Out, H: Fn(KeyEvent) -> Option<Out>, S: WidgetState<Out>> WidgetState<Out> for State<H, S> {
    fn reset_focus(&mut self) -> Focusable {
        Focusable::Yes
    }

    fn draw(&self, focus: Focus, area: Rect, buffer: &mut prelude::Buffer) -> Option<Position> {
        self.inner.draw(focus, area, buffer)
    }

    fn handle_key_event(
        &mut self,
        reactivity_nodes: &mut crate::reactivity::ReactivityNodes,
        event: KeyEvent,
    ) -> Option<Out> {
        self.inner
            .handle_key_event(reactivity_nodes, event)
            .or_else(|| (self.handler)(event))
    }
}

impl<Out, H: Copy + Fn(KeyEvent) -> Option<Out>, W: Widget<Out>> Widget<Out>
    for WithKeyHandler<H, W>
{
    type State = State<H, W::State>;

    fn init(&self, ctx: &mut Ctx) -> Self::State {
        State {
            handler: self.handler,
            inner: self.widget.init(ctx),
        }
    }

    fn update(&self, ctx: &mut Ctx, state: &mut Self::State) {
        state.handler = self.handler;
        self.widget.update(ctx, &mut state.inner);
    }
}

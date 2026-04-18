use crossterm::event::KeyEvent;
use ratatui::{
    buffer::Buffer,
    layout::{Position, Rect},
    widgets::Widget as _,
};

use crate::reactivity::{Ctx, Effect, ReactivityNodes};

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

    fn init(&self, ctx: &mut Ctx) -> Self::State;

    fn update(&self, ctx: &mut Ctx, state: &mut Self::State);
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

    fn init(&self, _: &mut Ctx) -> Self::State {
        self
    }

    fn update(&self, _: &mut Ctx, state: &mut Self::State) {
        // TODO: remove. This isn't a reactive widget.
        *state = self;
    }
}

pub struct TextWidgetState {
    text: String,
    effect: Effect,
}

impl<Out> WidgetState<Out> for TextWidgetState {
    fn reset_focus(&mut self) -> Focusable {
        Focusable::No
    }

    fn draw(&self, _: Focus, area: Rect, buffer: &mut Buffer) -> Option<Position> {
        (&*self.text).render(area, buffer);
        None
    }

    fn handle_key_event(&mut self, _: &mut ReactivityNodes, _: KeyEvent) -> Option<Out> {
        None
    }
}

impl<Out, F: Fn(&mut Ctx) -> String> Widget<Out> for F {
    type State = TextWidgetState;

    fn init(&self, ctx: &mut Ctx) -> Self::State {
        let effect = Effect::new(ctx.reactivity_nodes);

        let mut text = None;

        effect.call(ctx, |ctx| {
            text = Some(self(ctx));
        });

        TextWidgetState {
            text: text.unwrap(),
            effect,
        }
    }

    fn update(&self, ctx: &mut Ctx, state: &mut Self::State) {
        state.effect.call(ctx, |ctx| {
            state.text = self(ctx);
        });
    }
}

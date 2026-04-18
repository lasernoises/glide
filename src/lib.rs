use color_eyre::eyre::Result;
use crossterm::event::{self, Event, KeyCode};

use crate::widget::{Focus::*, Widget, WidgetState};

pub mod list_content;
pub mod reactivity;
pub mod widget;
pub mod widgets;

pub fn run(widget: impl Widget<()>) -> Result<()> {
    let mut state = widget.init();

    color_eyre::install()?;
    let mut terminal = ratatui::init();
    loop {
        terminal.draw(|frame| {
            let cursor_position = state.draw(Focused, frame.area(), frame.buffer_mut());

            if let Some(position) = cursor_position {
                frame.set_cursor_position(position);
            }
        })?;

        let event = event::read()?;

        if let Event::Key(event) = event {
            let handled = widget.handle_key_event(&mut state, event).is_some();

            if !handled && event.code == KeyCode::Char('q') {
                break;
            }

            widget.update(&mut state);
        }
    }
    ratatui::restore();
    Ok(())
}

use glide::{
    run,
    widgets::{
        state::{change, state},
        with_key_handler::with_key_handler,
    },
};

fn main() {
    run(state(
        || 0,
        |value| {
            with_key_handler(
                move |event| match event.code {
                    crossterm::event::KeyCode::Char('+') => Some(change(
                        glide::reactivity::PrimitiveChange::ReplaceBy(value + 1),
                    )),
                    crossterm::event::KeyCode::Char('-') => Some(change(
                        glide::reactivity::PrimitiveChange::ReplaceBy(value - 1),
                    )),
                    _ => None,
                },
                &*value.to_string().leak(),
            )
        },
    ))
    .unwrap();
}

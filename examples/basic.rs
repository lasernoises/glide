use glide::{
    reactivity::Ctx,
    run,
    widgets::{
        state::{change, state},
        with_key_handler::with_key_handler,
    },
};

fn main() {
    run(state(
        || 0i32,
        |value| {
            with_key_handler(
                move |event| match event.code {
                    crossterm::event::KeyCode::Char('+') => Some(change(
                        glide::reactivity::PrimitiveChange::ReplaceBy(value.read_untracked() + 1),
                    )),
                    crossterm::event::KeyCode::Char('-') => Some(change(
                        glide::reactivity::PrimitiveChange::ReplaceBy(value.read_untracked() - 1),
                    )),
                    _ => None,
                },
                move |ctx: &mut Ctx| value.read(ctx).to_string(),
            )
        },
    ))
    .unwrap();
}

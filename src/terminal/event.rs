pub use crossterm::{
    event,
    terminal::{disable_raw_mode, enable_raw_mode},
};

#[inline]
#[must_use]
pub const fn read(event: &event::Event) -> Option<(event::KeyCode, event::KeyModifiers)> {
    if let event::Event::Key(event::KeyEvent {
        code,
        modifiers,
        kind: event::KeyEventKind::Press,
        ..
    }) = event
    {
        Some((*code, *modifiers))
    } else {
        None
    }
}

#[macro_export]
macro_rules! r#continue {
    () => {
        (
            ::crossterm::event::KeyCode::Enter | ::crossterm::event::KeyCode::Char('\n'),
            _,
        )
    };
}
pub use r#continue;
#[macro_export]
macro_rules! back {
    () => {
        (
            ::crossterm::event::KeyCode::Char('d'),
            ::crossterm::event::KeyModifiers::CONTROL,
        ) | (
            ::crossterm::event::KeyCode::Esc,
            ::crossterm::event::KeyModifiers::NONE,
        )
    };
}
pub use back;
#[macro_export]
macro_rules! quit {
    () => {
        (
            ::crossterm::event::KeyCode::Char('c'),
            ::crossterm::event::KeyModifiers::CONTROL,
        ) | (::crossterm::event::KeyCode::Esc, _)
    };
}

pub use quit;

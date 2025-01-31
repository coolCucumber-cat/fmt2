use crate::{
    fmt,
    utils::{count_newlines, first_line_no_debug_assertion},
    write::{Flush, Write},
};

pub struct ScreenArea<'w, W>
where
    W: Write,
{
    pub cursor_y: u16,
    pub writer: &'w mut W,
}

impl<'w, W> ScreenArea<'w, W>
where
    W: Write,
{
    pub const fn new(writer: &'w mut W) -> Self {
        Self {
            cursor_y: 0,
            writer,
        }
    }
}

impl<W> ScreenArea<'_, W>
where
    W: Write,
{
    pub fn move_to_y(&mut self, y: u16) -> Result<(), W::Error> {
        match cmp_abs_diff(self.cursor_y, y) {
            Some((ordering, offset)) => {
                self.cursor_y = y;
                match ordering {
                    OrderingGreaterLess::Less => {
                        fmt! { (? self.writer) => @cursor_move(@down, {offset}) }
                    }
                    OrderingGreaterLess::Greater => {
                        fmt! { (? self.writer) => @cursor_move(@up, {offset}) }
                    }
                }
            }
            None => Ok(()),
        }
    }

    pub fn move_up(&mut self, up: u16) -> Result<(), W::Error> {
        self.cursor_y = self.cursor_y.saturating_sub(up);
        fmt! { (? self.writer) => @cursor_move(@up, {up})}
    }

    pub fn move_down(&mut self, down: u16) -> Result<(), W::Error> {
        self.cursor_y = self.cursor_y.saturating_add(down);
        fmt! { (? self.writer) => @cursor_move(@down, {down}) }
    }

    pub fn clear(&mut self) -> Result<(), W::Error> {
        self.clear_from(0)
    }

    pub fn clear_from(&mut self, y: u16) -> Result<(), W::Error> {
        self.move_to_y(y)?;
        fmt! { (? self.writer) => @cursor_move_to_x(@start) @clear(@from_cursor_down) }
    }

    pub fn write_str_first_line(&mut self, s: &str) -> Result<(), W::Error> {
        self.writer.write_str(first_line_no_debug_assertion(s))
    }
}

impl<W> Drop for ScreenArea<'_, W>
where
    W: Write,
{
    fn drop(&mut self) {
        let _ = self.clear();
    }
}

impl<W> Write for ScreenArea<'_, W>
where
    W: Write,
{
    type Error = W::Error;

    const IS_LINE_BUFFERED: bool = W::IS_LINE_BUFFERED;

    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        self.cursor_y = count_newlines(s)
            .try_into()
            .map(|n| self.cursor_y.saturating_add(n))
            .unwrap_or(u16::MAX);
        self.writer.write(s)
    }

    fn flush_hint(&mut self) {
        self.writer.flush_hint();
    }
}

impl<W> Flush for ScreenArea<'_, W>
where
    W: Flush + Write,
{
    type Error = <W as Flush>::Error;

    fn flush(&mut self) -> Result<(), Self::Error> {
        self.writer.flush()
    }
}

enum OrderingGreaterLess {
    Less,
    Greater,
}

#[must_use]
#[inline]
fn cmp_abs_diff(a: u16, b: u16) -> Option<(OrderingGreaterLess, u16)> {
    match a.checked_sub(b) {
        Some(0) => None,
        Some(offset) => Some((OrderingGreaterLess::Greater, offset)),
        None => Some((OrderingGreaterLess::Less, unsafe { b.unchecked_sub(a) })),
    }
}

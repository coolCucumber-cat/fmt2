use super::{utils::to_prompt_result_try, Back, Quit, Result as PromptResult};

use crate::{
    terminal::{event, screen_area::ScreenArea},
    write::Write,
};

use core::{
    ops::ControlFlow::{Break, Continue},
    str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnsignedInt<'q, F> {
    question: &'q str,
    validator: F,
}

impl<'q, F> UnsignedInt<'q, F> {
    #[must_use]
    pub const fn new_with(question: &'q str, validator: F) -> Self {
        Self {
            question,
            validator,
        }
    }

    #[must_use]
    #[inline]
    pub const fn question(&self) -> &'q str {
        self.question
    }

    #[must_use]
    #[inline]
    pub const fn validator(&self) -> &F {
        &self.validator
    }
}

impl<'q, TOut> UnsignedInt<'q, fn(TOut) -> Result<TOut, &'static str>> {
    #[must_use]
    pub const fn new(question: &'q str) -> Self {
        Self {
            question,
            validator: Ok,
        }
    }
}

impl<F> UnsignedInt<'_, F> {
    pub fn prompt<TIn, TOut>(&self) -> PromptResult<TOut, std::io::Error>
    where
        TIn: FromStr,
        F: Fn(TIn) -> Result<TOut, &'static str>,
    {
        self.prompt_with(&mut ScreenArea::new(&mut std::io::stdout()))
    }

    #[allow(clippy::cognitive_complexity)]
    pub fn prompt_with<W, TIn, TOut>(
        &self,
        screen_area: &mut ScreenArea<W>,
    ) -> PromptResult<TOut, W::Error>
    where
        W: Write<Error = std::io::Error>,
        TIn: FromStr,
        F: Fn(TIn) -> Result<TOut, &'static str>,
    {
        macro_rules! debug_assert_cursor_x_valid_get {
            ($cursor_x:expr, $digits:expr) => {
                debug_assert!(usize::from($cursor_x) < ($digits).len());
            };
        }
        macro_rules! debug_assert_cursor_x_valid_set {
            ($cursor_x:expr, $digits:expr) => {
                debug_assert!(usize::from($cursor_x) <= ($digits).len());
            };
        }

        macro_rules! fmt_value {
			($args:tt => $value:expr, cursor_x = $cursor_x:expr $(, start = $start:tt)? $(, end = $end:tt)? $(,)?) => {
				$crate::fmt! { $args =>
					$($start)?
					@cursor_hide
					@reset_line
					{@unique_20250115_1047 = $value; str first_line}
					@cursor_move_to_x({@unique_20250115_1150 = u16::saturating_add($cursor_x, 1)})
					@cursor_show
					$($end)?
				}
			};
		}

        fn write_replace_error<W>(
            screen_area: &mut ScreenArea<W>,
            error: &str,
            error_y: u16,
            cursor_x: u16,
            cursor_y: u16,
        ) -> Result<(), W::Error>
        where
            W: Write,
        {
            // at input position
            // crate::fmt! { (? screen_area) => @cursor_move_to_x(@start) }?;
            crate::fmt! { (? screen_area) => @cursor_hide @cursor_move_to_x(@start) }?;
            screen_area.move_to_y(error_y)?;
            // at error position
            crate::fmt_prompt_error! { (? screen_area) => {error}, start = [@clear(@from_cursor_down)] }?;
            screen_area.move_to_y(cursor_y)?;
            crate::fmt! { (? screen_area) => @cursor_move_to_x({cursor_x.saturating_add(1)}) @cursor_show }
            // Ok(())
            // at input position again
        }

        let y_start = screen_area.cursor_y;

        // string
        let mut digits: Vec<Digit> = vec![];
        let mut cursor_x: u16 = 0;

        // at question line
        to_prompt_result_try!(
            crate::fmt_prompt_question! { (? screen_area) => {self.question}, start = [@cursor_move_to_x(@start)], end = ln }
        );

        // at input line
        let input_row = screen_area.cursor_y;

        // help line
        to_prompt_result_try!(
            crate::fmt_prompt_help! { (? screen_area) => [@[super::consts::UNSIGNED_INT_HELP!()]], start = ln }
        );

        // error line
        let error_row: u16 = screen_area.cursor_y.saturating_add(1);

        // start
        to_prompt_result_try!(screen_area.move_to_y(input_row));
        to_prompt_result_try!(
            crate::fmt! { (? screen_area) => @cursor_move_to_x(@start) @cursor_show }
        );

        let t_out = loop {
            use crossterm::event::KeyCode as KC;

            let event = to_prompt_result_try!(crossterm::event::read());
            let event = event::read(&event);
            let Some(key) = event else {
                continue;
            };

            break match key {
                event::r#continue!() => {
                    let t_in = digits_to_number::<TIn>(&digits);
                    let t_out = match t_in {
                        Some(int) => (self.validator)(int),
                        None => Err("number too large"),
                    };
                    let t_out = match t_out {
                        Ok(t_out) => t_out,
                        Err(e) => {
                            to_prompt_result_try!(write_replace_error(
                                screen_area,
                                e,
                                error_row,
                                cursor_x,
                                input_row
                            ));
                            continue;
                        }
                    };
                    Continue(Continue(t_out))
                }
                event::back!() => Continue(Break(Back)),
                event::quit!() => Break(Ok(Quit)),

                (KC::Char(c), _) => {
                    // unless char is 0 to 9, skip
                    let Ok(d) = c.try_into() else {
                        continue;
                    };
                    debug_assert_cursor_x_valid_set!(cursor_x, digits);
                    digits.insert(cursor_x.into(), d);
                    cursor_x = cursor_x.saturating_add(1);
                    to_prompt_result_try!(
                        fmt_value! { (? screen_area) => digits, cursor_x = cursor_x }
                    );
                    continue;
                }
                (KC::Delete, _) => {
                    let cursor_x_usize = cursor_x.into();
                    // if no digit at cursor position (cursor at end), skip
                    if cursor_x_usize >= digits.len() {
                        continue;
                    }
                    debug_assert_cursor_x_valid_get!(cursor_x, digits);
                    digits.remove(cursor_x_usize);
                    to_prompt_result_try!(
                        fmt_value! { (? screen_area) => digits, cursor_x = cursor_x }
                    );
                    continue;
                }
                (KC::Backspace, _) => {
                    if digits.is_empty() || cursor_x == 0 {
                        continue;
                    }
                    cursor_x = unsafe { cursor_x.unchecked_sub(1) };
                    debug_assert_cursor_x_valid_get!(cursor_x, digits);
                    digits.remove(cursor_x.into());
                    to_prompt_result_try!(
                        fmt_value! { (? screen_area) => digits, cursor_x = cursor_x }
                    );
                    continue;
                }
                (KC::Left, _) => {
                    if cursor_x == 0 {
                        continue;
                    }
                    cursor_x = unsafe { cursor_x.unchecked_sub(1) };
                    debug_assert_cursor_x_valid_set!(cursor_x, digits);
                    to_prompt_result_try!(
                        crate::fmt! { (? screen_area) => @cursor_move_to_x({cursor_x.saturating_add(1)})}
                    );
                    continue;
                }
                (KC::Right, _) => {
                    if usize::from(cursor_x) >= digits.len() {
                        continue;
                    }
                    cursor_x = cursor_x.saturating_add(1);
                    debug_assert_cursor_x_valid_set!(cursor_x, digits);
                    to_prompt_result_try!(
                        crate::fmt! { (? screen_area) => @cursor_move_to_x({cursor_x.saturating_add(1)})}
                    );
                    continue;
                }
                _ => continue,
            };
        };

        to_prompt_result_try!(crate::fmt! { (? screen_area) => @cursor_hide });
        to_prompt_result_try!(screen_area.clear_from(y_start));
        t_out
    }
}

transmute_guard::enum_alias! {
    enum Digit: core::ascii::Char = {
        Digit0 |
        Digit1 |
        Digit2 |
        Digit3 |
        Digit4 |
        Digit5 |
        Digit6 |
        Digit7 |
        Digit8 |
        Digit9
    };
}

impl TryFrom<char> for Digit {
    type Error = ();

    #[inline]
    fn try_from(value: char) -> Result<Self, Self::Error> {
        Digit::try_from(char_try_into_ascii_char(value).ok_or(())?)
    }
}

#[inline]
fn char_try_into_ascii_char(char: char) -> Option<core::ascii::Char> {
    let u32 = u32::from(char);
    let u8 = u8::try_from(u32).ok()?;
    core::ascii::Char::from_u8(u8)
}

#[inline]
fn digits_to_number<T>(digits: &[Digit]) -> Option<T>
where
    T: FromStr,
{
    let s = transmute_guard::safe_transmute_ref::<[Digit], str>(digits);
    s.parse::<T>().ok()
}

#[cfg(test)]
#[test]
#[allow(
    clippy::allow_attributes,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::missing_const_for_fn,
    unused_variables,
    unused_imports,
    clippy::cognitive_complexity
)]
fn test() {
    {
        fn d(char: char) -> Digit {
            Digit::try_from(char).unwrap()
        }
        assert_eq!(digits_to_number::<u8>(&[]), Some(0));
        assert_eq!(digits_to_number::<u8>(&[d('0')]), Some(0));
        assert_eq!(digits_to_number::<u8>(&[d('1')]), Some(1));
        assert_eq!(digits_to_number::<u8>(&[d('1'), d('2')]), Some(12));
        assert_eq!(digits_to_number::<u8>(&[d('1'), d('2'), d('3')]), Some(123));
    }
    assert_eq!(Digit::try_from('0'), Ok(Digit::Digit0));
    assert_eq!(Digit::try_from('1'), Ok(Digit::Digit1));
    assert_eq!(Digit::try_from('2'), Ok(Digit::Digit2));
    assert_eq!(Digit::try_from('3'), Ok(Digit::Digit3));
    assert_eq!(Digit::try_from('4'), Ok(Digit::Digit4));
    assert_eq!(Digit::try_from('5'), Ok(Digit::Digit5));
    assert_eq!(Digit::try_from('6'), Ok(Digit::Digit6));
    assert_eq!(Digit::try_from('7'), Ok(Digit::Digit7));
    assert_eq!(Digit::try_from('8'), Ok(Digit::Digit8));
    assert_eq!(Digit::try_from('9'), Ok(Digit::Digit9));
    assert_eq!(Digit::try_from('a'), Err(()));

    assert_eq!(Digit::Digit0.as_u8(), 0);
    assert_eq!(Digit::Digit1.as_u8(), 1);
    assert_eq!(Digit::Digit2.as_u8(), 2);
    assert_eq!(Digit::Digit3.as_u8(), 3);
    assert_eq!(Digit::Digit4.as_u8(), 4);
    assert_eq!(Digit::Digit5.as_u8(), 5);
    assert_eq!(Digit::Digit6.as_u8(), 6);
    assert_eq!(Digit::Digit7.as_u8(), 7);
    assert_eq!(Digit::Digit8.as_u8(), 8);
    assert_eq!(Digit::Digit9.as_u8(), 9);
}

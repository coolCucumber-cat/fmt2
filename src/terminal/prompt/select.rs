use super::{utils::to_prompt_result_try, Back, PromptWith, Quit, Result as PromptResult};

use crate::{
    str::FmtStr,
    terminal::{event, screen_area::ScreenArea},
    write::Write,
};

use core::ops::{
    ControlFlow::{Break, Continue},
    Deref,
};

const MAX_LEN: usize = (u16::MAX / 2) as usize;

#[derive(Debug)]
pub struct Choices<T>([T]);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ChoicesError {
    IsEmpty,
    TooLong,
}
#[inline]
pub const fn check_choices_len(len: usize) -> Result<(), ChoicesError> {
    if len == 0 {
        Err(ChoicesError::IsEmpty)
    } else if len > MAX_LEN {
        Err(ChoicesError::TooLong)
    } else {
        Ok(())
    }
}
#[inline]
const fn unwrap_choices_check(r: Result<(), ChoicesError>) {
    match r {
        Ok(()) => {}
        Err(ChoicesError::IsEmpty) => panic!("choices is empty"),
        Err(ChoicesError::TooLong) => panic!("choices is too long"),
    }
}

impl<'c, T> Choices<T>
where
    Self: 'c,
{
    pub const fn check(&self) -> Result<(), ChoicesError> {
        check_choices_len(self.0.len())
    }

    pub const fn new<const LEN: usize>(choices: &'c [T; LEN]) -> &'c Self {
        const {
            unwrap_choices_check(check_choices_len(LEN));
        };
        let s = unsafe { Self::new_unchecked(choices) };

        #[cfg(debug_assertions)]
        {
            unwrap_choices_check(s.check());
        }
        s
    }

    pub const fn try_new(choices: &'c [T]) -> Result<&'c Self, ChoicesError> {
        let s = unsafe { Self::new_unchecked(choices) };
        match s.check() {
            Ok(()) => Ok(s),
            Err(e) => Err(e),
        }
    }

    /// new
    ///
    /// # Safety
    /// - choices must have at least len of 1
    /// - choice must not be too long
    #[must_use]
    pub const unsafe fn new_unchecked(choices: &'c [T]) -> &'c Self {
        let ptr = core::ptr::from_ref::<[T]>(choices) as *const Self;
        unsafe { &*ptr }
    }

    #[must_use]
    pub const fn get(&'c self) -> &'c [T] {
        &self.0
    }

    #[must_use]
    pub fn first(&'c self) -> &'c T {
        unsafe { self.0.get_unchecked(0) }
    }

    #[inline]
    #[must_use]
    #[expect(clippy::cast_possible_truncation, clippy::len_without_is_empty)]
    pub const fn len(&self) -> u16 {
        let len = self.0.len();
        #[cfg(debug_assertions)]
        #[expect(clippy::manual_assert)]
        {
            if len > (u16::MAX as usize) {
                panic!("len is not valid");
            }
        }
        len as u16
    }
}
impl<'c, T, const LEN: usize> From<&'c [T; LEN]> for &'c Choices<T> {
    fn from(value: &'c [T; LEN]) -> Self {
        Choices::new(value)
    }
}
impl<'c, T> TryFrom<&'c [T]> for &'c Choices<T> {
    type Error = ChoicesError;
    fn try_from(value: &'c [T]) -> Result<Self, Self::Error> {
        Choices::try_new(value)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Select<'q, 'c, T, const IS_MAIN_MENU: bool = false> {
    question: &'q str,
    choices: &'c Choices<T>,
}

impl<'q, 'c, T, U, const IS_MAIN_MENU: bool> Select<'q, 'c, T, IS_MAIN_MENU>
where
    T: Deref<Target = U>,
    U: FmtStr + ?Sized,
{
    pub const fn check(&self) -> Result<(), ChoicesError> {
        self.choices.check()
    }

    #[inline]
    #[must_use]
    pub const fn new(question: &'q str, choices: &'c Choices<T>) -> Self {
        Self { question, choices }
    }

    pub fn try_new(question: &'q str, choices: &'c [T]) -> Result<Self, ChoicesError> {
        let s = unsafe { Self::new_unchecked(question, choices) };
        s.check().map(|()| s)
    }

    /// new
    ///
    /// # Safety
    /// - question must have no newline
    /// - every choice must have no newline
    /// - choices must have at least len of 1
    pub const unsafe fn new_unchecked(question: &'q str, choices: &'c [T]) -> Self {
        Self {
            question,
            choices: Choices::new_unchecked(choices),
        }
    }

    #[must_use]
    #[inline]
    pub const fn question(&self) -> &'q str {
        self.question
    }

    #[inline]
    #[must_use]
    pub const fn choices(&self) -> &'c Choices<T> {
        self.choices
    }
}

impl<'c, W, T, U, const IS_MAIN_MENU: bool> PromptWith<W> for Select<'_, 'c, T, IS_MAIN_MENU>
where
    W: Write<Error = std::io::Error>,
    T: Deref<Target = U>,
    U: FmtStr + ?Sized,
{
    type Output = &'c T;

    #[allow(clippy::cognitive_complexity)]
    fn prompt_with(&self, screen_area: &mut ScreenArea<W>) -> PromptResult<Self::Output, W::Error> {
        // # Example
        // example of prompt with list length of 3
        //
        // 0:? question
        // 1:  answer 1
        // 2:> answer 2
        // 3:  answer 3
        // 4:

        macro_rules! fmt_prompt_unchosen_choice {
			($args:tt => $choice:expr $(, start = $start:tt)? $(, end = $end:tt)? $(,)?) => {
				$crate::fmt! { $args =>
					$($start)?
					@[$crate::terminal::prompt::consts::UNCHOSEN_CHOICE_LINE_PREFIX!()]
					{@unique_20250115_1039 = $choice; str first_line}
					$($end)?
				}
			};
		}

        macro_rules! fmt_prompt_chosen_choice {
			($args:tt => $choice:expr $(, start = $start:tt)? $(, end = $end:tt)? $(,)?) => {
				$crate::fmt! { $args =>
					$($start)?
					@fg(@cyan) [
						@[$crate::terminal::prompt::consts::CHOSEN_CHOICE_LINE_PREFIX!()]
						{@unique_20250115_1040 = $choice; str first_line}
					]
					$($end)?
				}
			};
		}

        #[cfg(debug_assertions)]
        #[expect(clippy::expect_used)]
        {
            self.check().expect("Select must be valid");
        }

        let y_start = screen_area.cursor_y;

        // indexes
        let len = self.choices.len();
        debug_assert_ne!(len, 0);
        let choices_end_index: u16 = unsafe { len.unchecked_sub(1) };
        let mut choice_index: u16 = 0;

        // ask question
        to_prompt_result_try!(
            crate::fmt_prompt_question! { (? screen_area) => {self.question}, start = [@cursor_move_to_x(@start)], end = ln }
        );

        let choices_row_offset = screen_area.cursor_y;

        // show first choice
        let mut choices_iter = self.choices.get().iter();
        let first_choice = {
            #[cfg(debug_assertions)]
            #[expect(clippy::expect_used)]
            {
                choices_iter
                    .next()
                    .expect("expected iter to have at least one item (len check failed)")
            }
            #[cfg(not(debug_assertions))]
            {
                let _ = choices_iter.next();
                unsafe { self.choices.get().get_unchecked(0) }
            }
        };
        to_prompt_result_try!(
            fmt_prompt_chosen_choice!((? screen_area) => first_choice, end = ln )
        );

        // show all other choices
        for choice in choices_iter {
            to_prompt_result_try!(
                fmt_prompt_unchosen_choice!((? screen_area) => choice, end = ln )
            );
        }

        if IS_MAIN_MENU {
            to_prompt_result_try!(
                crate::fmt_prompt_help!((? screen_area) => [@[crate::terminal::prompt::consts::MAIN_MENU_SELECT_HELP!()]] )
            );
        } else {
            to_prompt_result_try!(
                crate::fmt_prompt_help!((? screen_area) => [@[crate::terminal::prompt::consts::SELECT_HELP!()]] )
            );
        }

        // move cursor to first choice
        to_prompt_result_try!(screen_area.move_to_y(choices_row_offset));

        let v = loop {
            use crossterm::event::KeyCode as KC;

            let event = to_prompt_result_try!(crossterm::event::read());
            let Some(event) = event::read(&event) else {
                continue;
            };

            let choice_index_usize = usize::from(choice_index);

            let new_choice_index = match event {
                // continue (user has chosen a choice)
                event::r#continue!() => {
                    let choice = unsafe { self.choices.get().get_unchecked(choice_index_usize) };
                    break Continue(Continue(choice));
                }
                // back (user has gone back)
                event::back!() => {
                    break Continue(Break(Back));
                }
                // quit (user has quit)
                event::quit!() => {
                    break Break(Ok(Quit));
                }

                (KC::Up, _) => choice_index.checked_sub(1).unwrap_or(choices_end_index),
                (KC::Down, _) => {
                    if choice_index >= choices_end_index {
                        0
                    } else {
                        #[expect(clippy::arithmetic_side_effects)]
                        {
                            choice_index + 1
                        }
                    }
                }
                (KC::Home, _) => 0,
                (KC::End, _) => choices_end_index,

                // ignore
                _ => continue,
            };

            if choice_index == new_choice_index {
                continue;
            }

            // uncheck old choice
            let choice = unsafe { self.choices.get().get_unchecked(choice_index_usize) };
            to_prompt_result_try!(
                fmt_prompt_unchosen_choice! { (? screen_area) => choice, start = [@reset_line] }
            );

            // move to new choice
            #[cfg(debug_assertions)]
            {
                crate::utils::assert_add_no_overflow!(choices_row_offset, new_choice_index)
            };
            let new_cursor_y: u16 = choices_row_offset.saturating_add(new_choice_index);
            to_prompt_result_try!(screen_area.move_to_y(new_cursor_y));

            // check new choice
            let new_choice_index_usize = usize::from(new_choice_index);
            let new_choice = unsafe { self.choices.get().get_unchecked(new_choice_index_usize) };
            to_prompt_result_try!(
                fmt_prompt_chosen_choice! { (? screen_area) => new_choice, start = [@reset_line] }
            );

            choice_index = new_choice_index;
        };
        to_prompt_result_try!(screen_area.clear_from(y_start));
        v
    }
}

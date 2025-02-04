use super::{utils::to_prompt_result_try, Back, PromptWith, Quit, Result as PromptResult};

use crate::{
    terminal::{event, screen_area::ScreenArea},
    write::Write,
    write_to::Fmt,
};

use core::ops::ControlFlow::{Break, Continue};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Info<'i, 'a, I: ?Sized = str, A: ?Sized = str> {
    info: &'i I,
    additional: Option<&'a A>,
}
// pub struct Info<I = &'static str, A = &'static str> {
// 	info: I,
// 	additional: Option<A>,
// }

impl<'i, 'a, I: ?Sized, A: ?Sized> Info<'i, 'a, I, A> {
    #[inline]
    #[must_use]
    pub const fn new_with(info: &'i I, additional: Option<&'a A>) -> Self {
        Self { info, additional }
    }

    #[must_use]
    #[inline]
    pub const fn info(&self) -> &'i I {
        self.info
    }

    #[must_use]
    #[inline]
    pub const fn additional(&self) -> Option<&'a A> {
        self.additional
    }
}

impl<'i, I: ?Sized> Info<'i, '_, I> {
    #[inline]
    #[must_use]
    pub const fn new(info: &'i I) -> Self {
        Self {
            info,
            additional: None,
        }
    }
}

impl<W, I, A> PromptWith<W> for Info<'_, '_, I, A>
where
    W: Write<Error = std::io::Error>,
    I: Fmt + ?Sized,
    A: Fmt + ?Sized,
{
    type Output = ();

    fn prompt_with(&self, screen_area: &mut ScreenArea<W>) -> PromptResult<Self::Output, W::Error> {
        let y_start = screen_area.cursor_y;

        // print info
        to_prompt_result_try!(
            crate::fmt_prompt_question! { (? screen_area) => {self.info}, start = [@cursor_move_to_x(@start)], end = ln }
        );
        // print additional
        if let Some(a) = self.additional {
            to_prompt_result_try!(crate::fmt! { (? screen_area) => {a} ln });
        }
        to_prompt_result_try!(
            crate::fmt_prompt_help! { (? screen_area) => [@[crate::INFO_HELP!()]]}
        );

        let t0 = loop {
            // use crossterm::event::{KeyCode as KC, KeyModifiers as KM};

            let event = to_prompt_result_try!(crossterm::event::read());
            let event = event::read(&event);
            let Some(key) = event else {
                continue;
            };

            break match key {
                event::back!() => Continue(Break(Back)),
                event::quit!() => Break(Ok(Quit)),
                _ => Continue(Continue(())),
            };
        };

        to_prompt_result_try!(screen_area.clear_from(y_start));
        t0
    }
}

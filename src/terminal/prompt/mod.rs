use super::screen_area::ScreenArea;

use crate::write::Write;

use core::ops::ControlFlow;

pub mod consts;
#[cfg(feature = "info_prompt")]
pub mod info;
#[cfg(feature = "select_prompt")]
pub mod select;
#[cfg(feature = "unsigned_int_prompt")]
pub mod unsigned_int;
pub mod utils;

#[doc(alias = "PromptError")]
pub type Error = std::io::Error;

/// The user backed out of the menu
#[derive(Clone, Copy)]
#[must_use]
pub struct Back;

/// The user quit the program
#[derive(Clone, Copy)]
#[must_use]
pub struct Quit;

impl std::process::Termination for Quit {
    fn report(self) -> std::process::ExitCode {
        std::process::ExitCode::SUCCESS
    }
}

pub type Terminate<E> = core::result::Result<Quit, E>;

/// Result of interactions with the user in the terminal
/// # Variants
/// - ## [`Continue`]
///   - ### [`Continue(T)`](Continue)
///     The prompt was successful and we get the selected option
///   - ### [`Break(Back)`](Break)
///     The user backed out of the prompt and wants to return to the previous menu
/// - ## [`Break`]
///     The program has to be stopped
///   - ### [`Ok(Quit)`](Ok)
///     The user quit the program
///   - ### [`Err(PromptError)`](Err)
///     Some error occured and we should quit the program
#[doc(alias = "PromptResult")]
pub type Result<T, E> = ControlFlow<Terminate<E>, ControlFlow<Back, T>>;

pub trait PromptWith<W>
where
    W: Write,
{
    type Output;

    fn prompt_with(&self, screen_area: &mut ScreenArea<W>) -> Result<Self::Output, W::Error>;
}

pub trait Prompt<W>
where
    W: Write<Error = std::io::Error>,
{
    type Output;

    fn prompt(&self) -> Result<Self::Output, std::io::Error>;
}

impl<T> Prompt<std::io::Stdout> for T
where
    T: PromptWith<std::io::Stdout>,
{
    type Output = <T as PromptWith<std::io::Stdout>>::Output;

    fn prompt(&self) -> Result<Self::Output, std::io::Error> {
        self.prompt_with(&mut ScreenArea::new(&mut std::io::stdout()))
    }
}

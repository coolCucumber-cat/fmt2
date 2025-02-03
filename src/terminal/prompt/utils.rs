/// Works like [`?`] but for a [`TuiResult`]
#[macro_export]
macro_rules! prompt_result_try {
    ($expr:expr) => {{
        match $expr {
            ::core::ops::ControlFlow::Continue(::core::ops::ControlFlow::Continue(v)) => v,
            ::core::ops::ControlFlow::Continue(::core::ops::ControlFlow::Break(b)) => {
                return ::core::ops::ControlFlow::Continue(::core::ops::ControlFlow::Break(b))
            }
            ::core::ops::ControlFlow::Break(b) => return ::core::ops::ControlFlow::Break(b),
        }
    }};
}
pub use prompt_result_try;

/// Works like [`?`] but for a [`Result`](core::result::Result) which will be converted to a [`TuiResult`](Result)
#[macro_export]
#[doc(hidden)]
macro_rules! to_prompt_result_try {
    ($expr:expr) => {{
        match $expr {
            ::core::result::Result::Ok(v) => v,
            ::core::result::Result::Err(err) => {
                return ::core::ops::ControlFlow::Break(::core::result::Result::Err(err))
            }
        }
    }};
}
pub use to_prompt_result_try;

#[macro_export]
macro_rules! fmt_prompt_error {
	($args:tt => $error:tt $(, start = $start:tt)? $(, end = $end:tt)? $(,)?) => {
		$crate::fmt! { $args =>
			$($start)?
			@fg(@red) [
				@[$crate::ERROR_LINE_PREFIX!()]
				$error
			]
			$($end)?
		}
	};
}
pub use fmt_prompt_error;

#[macro_export]
macro_rules! fmt_prompt_question {
	($args:tt => $question:tt $(, start = $start:tt)? $(, end = $end:tt)? $(,)?) => {
		$crate::fmt! { $args =>
			$($start)?
			@fg(@green) [
				@[$crate::QUESTION_LINE_PREFIX!()]
				$question
			]
			$($end)?
		}
	};
}
pub use fmt_prompt_question;

#[macro_export]
macro_rules! fmt_prompt_help {
	($args:tt => $help:tt $(, start = $start:tt)? $(, end = $end:tt)? $(,)?) => {
		$crate::fmt! { $args =>
			$($start)?
			@fg(@cyan) [
				@[$crate::HELP_LINE_PREFIX!()]
				$help
				@[$crate::HELP_LINE_POSTFIX!()]
			]
			$($end)?
		}
	};
}
pub use fmt_prompt_help;

#[macro_export]
macro_rules! define_select_choices_enum {
	{
		$(#[$meta:meta])*
		$vis:vis $enum_name:ident {
			$($variant_name:ident => $str:expr),* $(,)?
		}
	} => {
		#[derive(Clone, Copy, PartialEq, Eq)]
		$(#[$meta:meta])*
		$vis enum $enum_name {
			$($variant_name),*
		}

		impl $enum_name {
			pub const CHOICES: &'static [Self] = &[$(Self::$variant_name),*];
		}

		impl $crate::str::FmtStaticStrImpl for $enum_name {
			#[inline]
			fn fmt_static_str_impl(&self) -> &'static str {
				match self {
					$(Self::$variant_name => $str),*
				}
			}
		}

		impl $crate::write_to::FmtAdvanced for $enum_name {
			type Target = str;
			#[inline]
			fn fmt_advanced(&self) -> &Self::Target {
				$crate::str::FmtStaticStrImpl::fmt_static_str_impl(self)
			}
		}
	};
}
pub use define_select_choices_enum;

#[macro_export]
macro_rules! select_choices_from {
	{ $($option:expr => $string:expr),* $(,)? } => {
		[$($crate::write_to::WithFmtAdvanced::new($option, $string)),*]
	};
}
pub use select_choices_from;

#[macro_export]
macro_rules! get_write_to_from_fmt_args {
    { $value:expr; { $($fmt_args:tt)* } } => {
        $crate::get_write_to_from_fmt_args! { $value; $($fmt_args)* }
    };
	{ $value:expr; noderef } => {
		$value
	};
    { $value:expr; advanced } => {{
        use $crate::write_to::FmtAdvanced as _;
		$value.fmt_advanced()
    }};
    { $value:expr; str } => {{
        use $crate::str::FmtStr as _;
		$value.fmt_str()
    }};
    { $value:expr; str first_line } => {{
        use $crate::str::FmtStr as _;
		$crate::utils::first_line($value.fmt_str())
    }};
    { $value:expr; str first_line no_debug_assertion } => {{
        use $crate::str::FmtStr as _;
		$crate::utils::first_line_no_debug_assertion($value.fmt_str())
    }};
    { $value:expr; .. } => {{
		use $crate::write_to::FmtIterator as _;
		use ::core::iter::IntoIterator as _;
		$value.into_iter().fmt_iterator()
    }};
    { $value:expr; } => {{
        use $crate::write_to::Fmt as _;
		$value.fmt()
    }};
    { $value:expr; ? } => {{
        use $crate::write_to::FmtDebug as _;
		$value.fmt_debug()
    }};
    { $value:expr; b } => {{
        use $crate::write_to::FmtBinary as _;
		$value.fmt_binary()
    }};
    { $value:expr; h } => {{
        use $crate::write_to::FmtHex as _;
		$value.fmt_hex()
    }};
    { $value:expr; .$PRECISION:expr } => {{
        use $crate::write_to::FmtPrecision as _;
		$value.fmt_precision::<$PRECISION>()
    }};
    { $value:expr; std } => {{
		use $crate::write_to::FmtStdDisplay as _;
		$value.fmt_std_display()
    }};
    { $value:expr; std? } => {{
		use $crate::write_to::FmtStdDebug as _;
		$value.fmt_std_debug()
    }};
    { $value:expr; std b } => {{
        use $crate::write_to::FmtStdBinary as _;
		$value.fmt_std_binary()
    }};
    { $value:expr; std h } => {{
		use $crate::write_to::FmtStdHex as _;
		$value.fmt_std_hex()
    }};
	{ $value:expr; std .$PRECISION:expr } => {{
		use $crate::write_to::FmtStdPrecision as _;
		$value.fmt_std_precision::<$PRECISION>()
	}};
}

#[doc(hidden)]
#[macro_export]
macro_rules! handle_write_error {
    ($result:expr => { break $lifetime:lifetime err }) => {
        if let ::core::result::Result::Err(err) = $result {
            break $lifetime::core::result::Result::Err(err);
        }
    };
    ($result:expr => { break $lifetime:lifetime }) => {
        if let ::core::result::Result::Err(_) = $result {
            break $lifetime;
        }
    };
    ($result:expr => { return err }) => {
        if let ::core::result::Result::Err(err) = $result {
            return ::core::result::Result::Err(err);
        }
    };
    ($result:expr => { return }) => {
        if let ::core::result::Result::Err(_) = $result {
            return;
        }
    };
    ($result:expr => { ? }) => {
        $result?;
    };
    ($result:expr => { ! }) => {
        match $result {
            ::core::result::Result::Ok(()) => {}
        }
    };
}

#[macro_export]
macro_rules! write_fmt_single_internal {
	($writer:expr => { $value:expr; $($fmt_args:tt)* } => $handle_error_args:tt) => {{
		$crate::handle_write_error! {
			$crate::write::Write::write_advanced::<_, false, false>(
				$writer,
				$crate::get_write_to_from_fmt_args! { $value; $($fmt_args)* },
			)
			=> $handle_error_args
		}
	}};

	($writer:expr => (@($($stmt:stmt)*)) => $handle_error_args:tt) => {
		$($stmt)*
	};

	($writer:expr => (@..($iterator:expr => |$name:ident $(: $ty:ty)?| $($fmt:tt)*)) => $handle_error_args:tt) => {{
		use ::core::iter::IntoIterator as _;
		for $name $(: $ty)? in $iterator.into_iter() {
			$crate::fmt_internal! {
				input: { $($fmt)* },
				output: {},
				args: {
					mode: nocapture write_inner {
						writer: $writer,
						handle_error_args: $handle_error_args,
					},
					ends_in_newline: false,
				}
			}
		}
	}};
	($writer:expr => (@..const($iterator:expr => $string:expr)) => $handle_error_args:tt) => {{
		use ::core::iter::ExactSizeIterator;
		let s: &str = $string;
		if s.len() != 0 {
			for _ in $iterator {
				$crate::handle_write_error! {
					$crate::write::Write::write_str($writer, s)
					=> $handle_error_args
				}
			}
		}
	}};

	($writer:expr => (@..join($iterator:expr => $join:tt => |$name:ident $(: $ty:ty)?| $($fmt:tt)*)) => $handle_error_args:tt) => {{
		use ::core::iter::IntoIterator as _;

		#[allow(irrefutable_let_patterns)]
		if let mut iterator = $iterator.into_iter() {
			if let ::core::option::Option::Some($name) = ::core::iter::Iterator::next(iterator) {
				$crate::fmt_internal! {
					input: { $($fmt)* },
					output: {},
					args: {
						mode: nocapture write_inner {
							writer: $writer,
							handle_error_args: $handle_error_args,
						},
						ends_in_newline: false,
					}
				}
				for $name $(: $ty)? in iterator {
					$crate::fmt_internal! {
						input: { $join $($fmt)* },
						output: {},
						args: {
							mode: nocapture write_inner {
								writer: $writer,
								handle_error_args: $handle_error_args,
							},
							ends_in_newline: false,
						}
					}
				}
			}
		}
	}};

	($writer:expr => [$("", )*] => $handle_error_args:tt) => {{
		::core::compile_error!("unreachable. dev error or bug using macro");
	}};

	($writer:expr => [$($value:expr, )+] => $handle_error_args:tt) => {{
		const S: &str = ::core::concat!($($value),+);
		if S.len() != 0 {
			$crate::handle_write_error! {
				$crate::write::Write::write_str($writer, S)
				=> $handle_error_args
			}
		}
	}};
}

#[macro_export]
macro_rules! len_hint_fmt_single_internal {
	({ $value:expr; $($fmt_args:tt)* }) => {
		$crate::write_to::WriteTo::len_hint(
			$crate::get_write_to_from_fmt_args! { $value; $($fmt_args)* }
		)
	};

	((@($($stmt:stmt)*))) => {
		0
	};

	((@..($iterator:expr => |$name:ident $(: $ty:ty)?| $($fmt:tt)*))) => {{
		0
	}};
	((@..const($iterator:expr => $string:expr))) => {{
		use ::core::iter::ExactSizeIterator;
		$string.len() * $iterator.len()
	}};

	((@..join($iterator:expr => $join:expr => |$name:ident $(: $ty:ty)?| $($fmt:tt)*))) => {{
		0
	}};

	([$("", )*]) => {{
		::core::compile_error!("unreachable. dev error or bug using macro");
	}};

	([$($value:expr, )+]) => {
		::core::concat!($($value),+).len()
	};
}

#[macro_export]
macro_rules! write_fmt_return_internal {
	($writer:expr => $($fmt:tt)*) => {
		$(
			$crate::write_fmt_single_internal! { $writer => $fmt => { return err } }
		)*
	};
}

#[macro_export]
macro_rules! len_hint_fmt_internal {
	($($fmt:tt)*) => {
		0
		$(
			+ $crate::len_hint_fmt_single_internal!($fmt)
		)*
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! fmt_internal {
	// region: do recursion

	// square brackets
	{
		input: { $([$($fmt_test:tt)*])+ $($inputs:tt)* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal! {
			input: { $($($fmt_test)*)+ $($inputs)* },
			output: { $($outputs)* },
			args: $args
		}
	};

	// ln
	{
		input: { $(@[$($prev:expr),* $(,)?])* ln $($inputs:tt)* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal! {
			input: { @[$($($prev, )*)* "\n"] $($inputs)* },
			output: { $($outputs)* },
			args: $args
		}
	};
	// literal
	{
		input: { $(@[$($prev:expr),* $(,)?])* $literal:literal $($inputs:tt)* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal! {
			input: { @[$($($prev, )*)* $literal] $($inputs)* },
			output: { $($outputs)* },
			args: $args
		}
	};
	// macro
	{
		input: { $(@[$($prev:expr),* $(,)?])* $macro_name:ident ! $macro_args:tt $($inputs:tt)* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal! {
			input: { @[$($($prev, )*)* $macro_name ! $macro_args] $($inputs)* },
			output: { $($outputs)* },
			args: $args
		}
	};
	// foreground ansi
	{
		input: { $(@[$($prev:expr),* $(,)?])* @fg(@$fg:tt) $inputs0:tt $($inputs:tt)* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal! {
			input: { @[$($($prev, )*)*] @fg_no_reset(@$fg) $inputs0 @fg_no_reset(@reset) $($inputs)* },
			output: { $($outputs)* },
			args: $args
		}
	};
	// background ansi
	{
		input: { $(@[$($prev:expr),* $(,)?])* @bg(@$bg:tt) $inputs0:tt $($inputs:tt)* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal! {
			input: { @[$($($prev, )*)*] @bg_no_reset(@$bg) $inputs0 @bg_no_reset(@reset) $($inputs)* },
			output: { $($outputs)* },
			args: $args
		}
	};
	// foreground ansi
	{
		input: { $(@[$($prev:expr),* $(,)?])* @fg_no_reset(@$fg:tt) $($inputs:tt)* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal! {
			input: { @[$($($prev, )*)* $crate::ansi_set_style!(foreground $fg)] $($inputs)* },
			output: { $($outputs)* },
			args: $args
		}
	};
	// background ansi
	{
		input: { $(@[$($prev:expr),* $(,)?])* @bg_no_reset(@$bg:tt) $($inputs:tt)* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal! {
			input: { @[$($($prev, )*)* $crate::ansi_set_style!(background $bg)] $($inputs)* },
			output: { $($outputs)* },
			args: $args
		}
	};
	// cursor show ansi
	{
		input: { $(@[$($prev:expr),* $(,)?])* @cursor_show $($inputs:tt)* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal! {
			input: { @[$($($prev, )*)* $crate::ANSI_START_macro!(), "?25h"] $($inputs)* },
			output: { $($outputs)* },
			args: $args
		}
	};
	// cursor hide ansi
	{
		input: { $(@[$($prev:expr),* $(,)?])* @cursor_hide $($inputs:tt)* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal! {
			input: { @[$($($prev, )*)* $crate::ANSI_START_macro!(), "?25l"] $($inputs)* },
			output: { $($outputs)* },
			args: $args
		}
	};
	// cursor move ansi
	{
		input: { $(@[$($prev:expr),* $(,)?])* @cursor_move(@$direction:tt, $count:tt) $($inputs:tt)* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal! {
			input: { @[$($($prev, )*)* $crate::ANSI_START_macro!()] $count @[$crate::ansi_direction_code!($direction)] $($inputs)* },
			output: { $($outputs)* },
			args: $args
		}
	};
	// cursor move_to_x ansi
	{
		input: { $(@[$($prev:expr),* $(,)?])* @cursor_move_to_x(@start) $($inputs:tt)* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal! {
			input: { @[$($($prev, )*)* $crate::ANSI_START_macro!(), 1, "G"] $($inputs)* },
			output: { $($outputs)* },
			args: $args
		}
	};
	// cursor move_to_x ansi
	{
		input: { $(@[$($prev:expr),* $(,)?])* @cursor_move_to_x($x:tt) $($inputs:tt)* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal! {
			input: { @[$($($prev, )*)* $crate::ANSI_START_macro!()] $x @["G"] $($inputs)* },
			output: { $($outputs)* },
			args: $args
		}
	};
	// cursor move_to_y ansi
	{
		input: { $(@[$($prev:expr),* $(,)?])* @cursor_move_to_y(@start) $($inputs:tt)* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal! {
			input: { @[$($($prev, )*)* $crate::ANSI_START_macro!(), 1, "d"] $($inputs)* },
			output: { $($outputs)* },
			args: $args
		}
	};
	// cursor move_to_y ansi
	{
		input: { $(@[$($prev:expr),* $(,)?])* @cursor_move_to_y($y:tt) $($inputs:tt)* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal! {
			input: { @[$($($prev, )*)* $crate::ANSI_START_macro!()] $y @["d"] $($inputs)* },
			output: { $($outputs)* },
			args: $args
		}
	};
	// cursor move_to ansi
	{
		input: { $(@[$($prev:expr),* $(,)?])* @cursor_move_to(@start, @start) $($inputs:tt)* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal! {
			input: { @[$($($prev, )*)* $crate::ANSI_START_macro!(), 1, ";", 1, "H"] $($inputs)* },
			output: { $($outputs)* },
			args: $args
		}
	};
	{
		input: { $(@[$($prev:expr),* $(,)?])* @cursor_move_to(@start, $y:tt) $($inputs:tt)* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal! {
			input: { @[$($($prev, )*)* $crate::ANSI_START_macro!()] $y @[";", 1, "H"] $($inputs)* },
			output: { $($outputs)* },
			args: $args
		}
	};
	{
		input: { $(@[$($prev:expr),* $(,)?])* @cursor_move_to($x:tt, @start) $($inputs:tt)* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal! {
			input: { @[$($($prev, )*)* $crate::ANSI_START_macro!(), 1, ";"] $x @["H"] $($inputs)* },
			output: { $($outputs)* },
			args: $args
		}
	};
	{
		input: { $(@[$($prev:expr),* $(,)?])* @cursor_move_to($x:tt, $y:tt) $($inputs:tt)* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal! {
			input: { @[$($($prev, )*)* $crate::ANSI_START_macro!()] $y @[";"] $x @["H"] $($inputs)* },
			output: { $($outputs)* },
			args: $args
		}
	};
	// enter_alt_screen ansi
	{
		input: { $(@[$($prev:expr),* $(,)?])* @enter_alt_screen $($inputs:tt)* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal! {
			input: { @[$($($prev, )*)* $crate::ANSI_START_macro!(), "?1049h"] $($inputs)* },
			output: { $($outputs)* },
			args: $args
		}
	};
	// leave_alt_screen ansi
	{
		input: { $(@[$($prev:expr),* $(,)?])* @leave_alt_screen $($inputs:tt)* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal! {
			input: { @[$($($prev, )*)* $crate::ANSI_START_macro!(), "?1049l"] $($inputs)* },
			output: { $($outputs)* },
			args: $args
		}
	};
	// clear ansi
	{
		input: { $(@[$($prev:expr),* $(,)?])* @clear(@$mode:ident) $($inputs:tt)* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal! {
			input: { @[$($($prev, )*)* $crate::ANSI_START_macro!(), $crate::ansi_clear_code!($mode)] $($inputs)* },
			output: { $($outputs)* },
			args: $args
		}
	};
	// reset_line ansi
	{
		input: { $(@[$($prev:expr),* $(,)?])* @reset_line $($inputs:tt)* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal! {
			input: { @[$($($prev, )*)*] @cursor_move_to_x(@start) @clear(@current_line) $($inputs)* },
			output: { $($outputs)* },
			args: $args
		}
	};
	// empty group of literals
	{
		input: { @[$(""),* $(,)?] $($inputs:tt)* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal! {
			input: { $($inputs)* },
			output: { $($outputs)* },
			args: $args
		}
	};
	// literal in a capturing expression (capturing syntax) (mode = capture)
	{
		input: { $(@[$($prev:expr),* $(,)?])* { @ $field_name:ident $(: $ty:ty)? = $literal:literal $(;)? } $($inputs:tt)* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal! {
			input: { @[$($($prev, )*)* $literal] $($inputs)* },
			output: { $($outputs)* },
			args: $args
		}
	};
	// literal in a non-capturing expression (mode = nocapture)
	{
		input: { $(@[$($prev:expr),* $(,)?])* { $literal:literal $(;)? } $($inputs:tt)* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal! {
			input: { @[$($($prev, )*)* $literal] $($inputs)* },
			output: { $($outputs)* },
			args: $args
		}
	};
	// 2 groups of literals
	{
		input: { @[$($literal1:expr),* $(,)?] @[$($literal2:expr),* $(,)?] $($inputs:tt)* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal! {
			input: { @[$($literal1, )* $($literal2),*] $($inputs)* },
			output: { $($outputs)* },
			args: $args
		}
	};
	// literals
	{
		input: { @[$($literal:expr),* $(,)?] $($inputs:tt)* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal! {
			input: { $($inputs)* },
			output: { $($outputs)* internal [$($literal, )*] },
			args: $args
		}
	};

	// capturing expression with generic type (mode = capture)
	{
		input: { { @ $field_name:ident = $value:expr $(; $($fmt_args:tt)*)? } $($inputs:tt)* },
		output: { $($outputs:tt)* },
		args: {
			mode: capture generate {
				lifetime: $lifetime:lifetime,
				optional_lifetime: $($optional_lifetime:lifetime)?,
				reference: { $($reference:tt)? },
			},
			$($rest:tt)*
		}
	} => {
		$crate::fmt_internal! {
			input: { $($inputs)* },
			output: { $($outputs)*
				external { $field_name : &$lifetime $field_name => $field_name = $value; { $($($fmt_args)*)? } => noderef }
			},
			args: {
				mode: capture generate {
					lifetime: $lifetime,
					optional_lifetime: $lifetime,
					reference: { $($reference)? },
				},
				$($rest)*
			}
		}
	};
	// capturing expression with concrete type (mode = capture)
	{
		input: { { @ $field_name:ident : $ty:ty = $value:expr $(; $($fmt_args:tt)*)? } $($inputs:tt)* },
		output: { $($outputs:tt)* },
		args: {
			mode: capture $output_mode:tt $args:tt,
			$($rest:tt)*
		}
	} => {
		$crate::fmt_internal! {
			input: { $($inputs)* },
			output: { $($outputs)*
				external { $field_name : $ty = $value; noderef => { $($($fmt_args)*)? } }
			},
			args: {
				mode: capture $output_mode $args,
				$($rest)*
			}
		}
	};
	// capturing expression (capturing syntax only, may be non-capturing) using variable as name and as value
	{
		input: { { @ $field_name:ident $(: $ty:ty)? $(; $($fmt_args:tt)*)? } $($inputs:tt)* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal! {
			input: { { @ $field_name $(: $ty)? = $field_name; $($($fmt_args)*)? } $($inputs)* },
			output: { $($outputs)* },
			args: $args
		}
	};
	// non-capturing expression (with capturing syntax) (mode = nocapture)
	{
		input: { { @ $field_name:ident $(: $ty:ty)? = $value:expr $(; $($fmt_args:tt)*)? } $($inputs:tt)* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal! {
			input: { $($inputs)* },
			output: { $($outputs)* internal { $value; $($($fmt_args)*)? } },
			args: $args
		}
	};
	// non-capturing expression (mode = nocapture)
	{
		input: { { $value:expr $(; $($fmt_args:tt)*)? } $($inputs:tt)* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal! {
			input: { $($inputs)* },
			output: { $($outputs)* internal { $value; $($($fmt_args)*)? } },
			args: $args
		}
	};

	// do
	{
		input: { @($($stmt:stmt)*) $($inputs:tt)* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal! {
			input: { $($inputs)* },
			output: { $($outputs)* internal (@($($stmt)*)) },
			args: $args
		}
	};
	// iter
	{
		input: { @..($iterator:expr => $($tt:tt)*) $($inputs:tt)* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal! {
			input: { $($inputs)* },
			output: { $($outputs)* internal (@..($iterator => $($tt)*)) },
			args: $args
		}
	};
	// iter const
	{
		input: { @..const($iterator:expr => $($tt:tt)*) $($inputs:tt)* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal! {
			input: { $($inputs)* },
			output: { $($outputs)* internal (@..const($iterator => $($tt)*)) },
			args: $args
		}
	};
	// iter join
	{
		input: { @..join($iterator:expr => $join:tt => $($tt:tt)*) $($inputs:tt)* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal! {
			input: { $($inputs)* },
			output: { $($outputs)* internal (@..join($iterator => $join => $($tt)*)) },
			args: $args
		}
	};

	// error (macros must be in square brackets)
	{
		input: { $name:ident!$tt:tt $($inputs:tt)* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		::core::compile_error!(::core::concat!(
			"TODO: macros must be in @[square brackets]\n",
			::core::stringify!($name), "!", ::core::stringify!($tt),
		))
	};
	// error
	{
		input: { $tt:tt $($inputs:tt)* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		::core::compile_error!(::core::concat!(
			"TODO: expressions must be either valid literals or in (round), {curly} or [square] brackets\n",
			"see documentation for the `fmt` macro\n",
			::core::stringify!($tt), "\n",
			$(
				::core::stringify!($inputs), "\n"
			),*
		))
	};

	// endregion

	// region: terminate recursion

	// (mode = nocapture write)
	{
		input: {},
		output: { $($(internal $fmt:tt)+)? },
		args: {
			mode: nocapture write {
				writer: $writer:expr,
				ignore_err: false,
			},
			ends_in_newline: $ends_in_newline:expr,
		}
	} => {
		'block: {
			$(
				use $crate::write::GetWriteInternal as _;
				#[allow(irrefutable_let_patterns)]
				if let writer = $writer.get_write_internal() {
					$(
						$crate::write_fmt_single_internal! { writer => $fmt => { break 'block err } }
					)+
					$crate::write::Write::flush_hint_advanced::<true, $ends_in_newline>(writer);
				}
			)?
			::core::result::Result::Ok(())
		}
	};
	// ignore error (mode = nocapture write)
	{
		input: {},
		output: { $($(internal $fmt:tt)+)? },
		args: {
			mode: nocapture write {
				writer: $writer:expr,
				ignore_err: true,
			},
			ends_in_newline: $ends_in_newline:expr,
		}
	} => {
		'block: {
			$(
				use $crate::write::GetWriteInternal as _;
				#[allow(irrefutable_let_patterns)]
				if let writer = $writer.get_write_internal() {
					$(
						$crate::write_fmt_single_internal! { writer => $fmt => { break 'block } }
					)+
					$crate::write::Write::flush_hint_advanced::<true, $ends_in_newline>(writer);
				}
			)?
		}
	};
	// (mode = nocapture write_inner)
	{
		input: {},
		output: { $(internal $fmt:tt)* },
		args: {
			mode: nocapture write_inner {
				writer: $writer:expr,
				handle_error_args: $handle_error_args:tt,
			},
			ends_in_newline: $ends_in_newline:expr,
		}
	} => {
		$(
			$crate::write_fmt_single_internal! { $writer => $fmt => $handle_error_args }
		)*
	};


	// empty string (mode = _ generate)
	{
		input: {},
		output: {},
		args: {
			mode: $capture_mode:tt generate $args:tt,
			$($rest:tt)*
		}
	} => {
		""
	};
	// only literals (concatable) (mode = _ generate)
	{
		input: {},
		output: { internal [$($literals:expr, )*] },
		args: {
			mode: $capture_mode:tt generate $args:tt,
			$($rest:tt)*
		}
	} => {
		::core::concat!($($literals),*)
	};


	// error (mode = nocapture generate)
	{
		input: {},
		output: { $(internal $fmt:tt)* },
		args: {
			mode: nocapture generate {
				name: self,
				ty: $ty:ty,
				value: $value:expr,
			},
			$($rest:tt)*
		}
	} => {
		compile_error!("not allowed to use name `self`. please specify a different name, such as `_self`")
	};
	// (mode = nocapture generate)
	{
		input: {},
		output: { $(internal $fmt:tt)* },
		args: {
			mode: nocapture generate {
				name: $name:ident,
				ty: Self,
				value: $value:expr,
			},
			$($rest:tt)*
		}
	} => {
		compile_error!("not allowed to use type `Self`. please specify the concrete type.");
	};
	// (mode = nocapture generate)
	{
		input: {},
		output: { $(internal $fmt:tt)* },
		args: {
			mode: nocapture generate {
				name: $name:ident,
				ty: $ty:ty,
				value: $value:expr,
			},
			ends_in_newline: $ends_in_newline:expr,
		}
	} => {{
		struct W($ty);

		impl $crate::write_to::WriteTo for W {
			#[inline]
			fn write_to<W>(&self, writer: &mut W) -> Result<(), W::Error>
			where
			W: $crate::write::Write + ?Sized,
			{
				let $name: &$ty = &self.0;
				$crate::write_fmt_return_internal!(
					writer =>
					$($fmt)*
				);
				::core::result::Result::Ok(())
			}
		}

		#[inline]
		fn new(t: &$ty) -> &W {
			unsafe { &*(t as *const $ty as *const W) }
		}

		new($value)
	}};

	// (mode = nocapture generate { to_string })
	{
		input: {},
		output: { $(internal $fmt:tt)* },
		args: {
			mode: nocapture generate {
				to_string
			},
			ends_in_newline: $ends_in_newline:expr,
		}
	} => {{
		let mut string = String::with_capacity($crate::len_hint_fmt_internal!($($fmt)*));
		$(
			$crate::write_fmt_single_internal! { &mut string => $fmt => { ! } }
		)*
		string
	}};

	// (mode = nocapture generate_methods)
	{
		input: {},
		output: { $(internal $fmt:tt)* },
		args: {
			mode: nocapture generate_methods {
				name: $name:pat,
			},
			ends_in_newline: $ends_in_newline:expr,
		}
	} => {
		#[inline]
		fn write_to<W>(&self, w: &mut W) -> ::core::result::Result<(), W::Error>
			where
				W: $crate::write::Write + ?Sized {
			let $name = self;
			$crate::write_fmt_return_internal!(
				w =>
				$($fmt)*
			);
			::core::result::Result::Ok(())
		}

		#[inline]
		fn len_hint(&self) -> usize {
			#[allow(unused_variables)]
			let $name = self;
			$crate::len_hint_fmt_internal!(
				$($fmt)*
			)
		}
	};

	// one capturing expression (mode = capture generate)
	{
		input: {},
		output: {
			external { $field_name:ident : $ty:ty $(=> $generic:ident)? = $value:expr; $fmt_args:tt => $fmt_args_2:tt }
		},
		args: {
			mode: capture generate {
				lifetime: $lifetime:lifetime,
				optional_lifetime: $($optional_lifetime:lifetime)?,
				reference: { $($reference:tt)? },
			},
			ends_in_newline: $ends_in_newline:expr,
		}
	} => {
		$crate::get_write_to_from_fmt_args! { $value; $fmt_args }
	};
	// one capturing expression and any amount of non-capturing expressions/literals (mode = capture generate)
	{
		input: {},
		output: {
			$(internal $internal0:tt)*
			external { $field_name:ident : $ty:ty => $generic:ident = $value:expr; $fmt_args:tt => $fmt_args_2:tt }
			$(internal $internal:tt)*
		},
		args: {
			mode: capture generate {
				lifetime: $lifetime:lifetime,
				optional_lifetime: $($optional_lifetime:lifetime)?,
				reference: { & },
			},
			ends_in_newline: $ends_in_newline:expr,
		}
	} => {
		{
			// for syntax highlighting
			#[allow(unused)]
			{
				let $field_name: ();
			}

			#[allow(non_camel_case_types)]
			struct W<$generic : $crate::write_to::WriteTo + ?Sized> {
				$field_name : $generic,
			}

			#[allow(non_camel_case_types)]
			impl<$generic : $crate::write_to::WriteTo + ?Sized> $crate::write_to::WriteTo for W<$generic> {
				const ENDS_IN_NEWLINE: bool = $ends_in_newline;

				#[inline]
				fn write_to<W>(&self, w: &mut W) -> ::core::result::Result<(), W::Error>
					where
						W: $crate::write::Write + ?Sized {
					$crate::write_fmt_return_internal!(
						w =>
						$($internal0)*
						{ &self.$field_name; $fmt_args_2 }
						$($internal)*
					);
					::core::result::Result::Ok(())
				}

				#[inline]
				fn len_hint(&self) -> usize {
					$crate::len_hint_fmt_internal!(
						$($internal0)*
						{ &self.$field_name; $fmt_args_2 }
						$($internal)*
					)
				}
			}

			#[inline]
			fn new<T>(t: &T) -> &W<T> where T: $crate::write_to::WriteTo + ?Sized {
				unsafe { &*(t as *const T as *const W<T>) }
			}

			new($crate::get_write_to_from_fmt_args! { $value; $fmt_args })
		}
	};
	// any amount of anything (mode = capture generate)
	{
		input: {},
		output: {
			$(internal $internal0:tt)*
			$(
				external { $field_name:ident : $ty:ty $(=> $generic:ident)? = $value:expr; $fmt_args:tt => $fmt_args_2:tt }
				$(internal $internal:tt)*
			)*
		},
		args: {
			mode: capture generate {
				lifetime: $lifetime:lifetime,
				optional_lifetime: $($optional_lifetime:lifetime)?,
				reference: { $($reference:tt)? },
			},
			ends_in_newline: $ends_in_newline:expr,
		}
	} => {
		$($reference)? {
			// for syntax highlighting
			#[allow(unused)]
			{
				$(let $field_name: ();)*
			}

			#[allow(non_camel_case_types)]
			struct W<$($optional_lifetime,)? $($($generic : $crate::write_to::WriteTo + ?Sized, )?)*> {
				$($field_name : $ty ),*
			}

			#[allow(non_camel_case_types)]
			impl<$($optional_lifetime,)? $($($generic : $crate::write_to::WriteTo + ?Sized, )?)*> $crate::write_to::WriteTo for W<$($optional_lifetime,)? $($($generic, )?)*> {
				const ENDS_IN_NEWLINE: bool = $ends_in_newline;

				#[inline]
				fn write_to<W>(&self, w: &mut W) -> ::core::result::Result<(), W::Error>
					where
						W: $crate::write::Write + ?Sized {
					$crate::write_fmt_return_internal!(
						w =>
						$($internal0)*
						$(
							{ self.$field_name; $fmt_args_2 }
							$($internal)*
						)*
					);
					::core::result::Result::Ok(())
				}

				#[inline]
				fn len_hint(&self) -> usize {
					$crate::len_hint_fmt_internal!(
						$($internal0)*
						$(
							{ self.$field_name; $fmt_args_2 }
							$($internal)*
						)*
					)
				}
			}

			W {
				$($field_name : $crate::get_write_to_from_fmt_args! { $value; $fmt_args }, )*
			}
		}
	};
	// endregion
}

#[macro_export]
macro_rules! fmt {
	{ {} => $($tt:tt)* } => {
		$crate::fmt_internal! {
			input: { $($tt)* },
			output: {},
			args: {
				mode: capture generate {
					lifetime: 'a,
					optional_lifetime:,
					reference: { & },
				},
				ends_in_newline: false,
			}
		}
	};
	{ { str } => $($tt:tt)* } => {
		$crate::fmt_internal! {
			input: { $($tt)* },
			output: {},
			args: {
				mode: nocapture generate {
					to_string
				},
				ends_in_newline: false,
			}
		}
	};
	{ { noref } => $($tt:tt)* } => {
		$crate::fmt_internal! {
			input: { $($tt)* },
			output: {},
			args: {
				mode: capture generate {
					lifetime: 'a,
					optional_lifetime:,
					reference: {},
				},
				ends_in_newline: false,
			}
		}
	};
	{ { $name:ident : $ty:ty } => $($tt:tt)* } => {
		$crate::fmt! { { $name : $ty = $name} => $($tt)* }
	};
	{ { $name:ident : $ty:ty = $value:expr } => $($tt:tt)* } => {
		$crate::fmt_internal! {
			input: { $($tt)* },
			output: {},
			args: {
				mode: nocapture generate {
					name: $name,
					ty: $ty,
					value: $value,
				},
				ends_in_newline: false,
			}
		}
	};
	{ (? #) => $($tt:tt)* } => {
		$crate::fmt! { (? ::std::io::stdout()) => $($tt)* }
	};
	{ (#) => $($tt:tt)* } => {
		$crate::fmt! { (::std::io::stdout()) => $($tt)* }
	};
	{ (? #err) => $($tt:tt)* } => {
		$crate::fmt! { (? ::std::io::stderr()) => $($tt)* }
	};
	{ (#err) => $($tt:tt)* } => {
		$crate::fmt! { (::std::io::stderr()) => $($tt)* }
	};
	{ (? #lock) => $($tt:tt)* } => {
		$crate::fmt! { (? std::io::Stdout::lock(&::std::io::stdout())) => $($tt)* }
	};
	{ (#lock) => $($tt:tt)* } => {
		$crate::fmt! { (std::io::Stdout::lock(&::std::io::stdout())) => $($tt)* }
	};
	{ (? #err lock) => $($tt:tt)* } => {
		$crate::fmt! { (? std::io::Stdout::lock(&::std::io::stderr())) => $($tt)* }
	};
	{ (#err lock) => $($tt:tt)* } => {
		$crate::fmt! { (std::io::Stdout::lock(&::std::io::stderr())) => $($tt)* }
	};
	{ (? $writer:expr) => $($tt:tt)* } => {
		$crate::fmt_internal! {
			input: { $($tt)* },
			output: {},
			args: {
				mode: nocapture write {
					writer: $writer,
					ignore_err: false,
				},
				ends_in_newline: false,
			}
		}
	};
	{ ($writer:expr) => $($tt:tt)* } => {
		$crate::fmt_internal! {
			input: { $($tt)* },
			output: {},
			args: {
				mode: nocapture write {
					writer: $writer,
					ignore_err: true,
				},
				ends_in_newline: false,
			}
		}
	};
	{ [$name:pat] => $($tt:tt)* } => {
		$crate::fmt_internal! {
			input: { $($tt)* },
			output: {},
			args: {
				mode: nocapture generate_methods {
					name: $name,
				},
				ends_in_newline: false,
			}
		}
	};
}

#[macro_export]
macro_rules! fmt_struct {
	($fmt:tt => $name:ident; { $field0:ident : $tt0:tt $(, $field:ident : $tt:tt)* $(,)? }) => {
		$crate::fmt! { $fmt => @[stringify!($name)] " { " @[stringify!($field0)] ": " $tt0 $(", " @[stringify!($field)] ": " $tt)* " }" }
	};

	($fmt:tt => $name:ident; { $field0:ident $(, $field:ident)* $(,)? }) => {
		$crate::fmt! { $fmt => @[stringify!($name)] " { " @[stringify!($field0)] ": " {$field0} $(", " @[stringify!($field)] ": " {$field})* " }" }
	};

	($fmt:tt => $name:ident; {}) => {
		$crate::fmt! { $fmt => @[stringify!($name)] " {}" }
	};
}

// #[macro_export]
// macro_rules! fmt_struct_debug {
// 	($fmt:tt => $name:ident; { $field0:ident $(, $field:ident)* $(,)? }) => {
// 		$crate::fmt! { $fmt => [stringify!($name)] " { " [stringify!($field0)] ": " {$field0;?} $(", " [stringify!($field)] ": " {$field;?})* " }" }
// 	};
//
// 	($fmt:tt => $name:ident; {}) => {
// 		$crate::fmt! { $fmt => [stringify!($name)] " {}" }
// 	};
// }

#[macro_export]
macro_rules! fmt_tuple_struct {
	($fmt:tt => $($name:ident;)? ($tt0:tt $(, $tt:tt)* $(,)?) ) => {
		$crate::fmt! { $fmt => $(@[stringify!($name)])? "(" $tt0 $(", " $tt)* ")" }
	};

	($fmt:tt => $($name:ident;)? () ) => {
		$crate::fmt! { $fmt => $(@[stringify!($name)])? "()" }
	};
}

#[macro_export]
macro_rules! fmt_unit_struct {
    ($name:ident) => {
        stringify!($name)
    };
}

#[macro_export]
macro_rules! fmt_unit_struct2 {
    ($name:ident) => {
        stringify!($name)
    };
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
    unused_imports
)]
pub fn test() {
    use crate::{
        write::{GetWriteInternal, Write},
        write_to::{Fmt, FmtDebug, ToString, WriteTo},
    };

    use core::ops::Deref;

    const fn ends_in_newline<W>(w: &W) -> bool
    where
        W: WriteTo + ?Sized,
    {
        W::ENDS_IN_NEWLINE
    }

    struct Struct {
        a: i32,
        b: bool,
    }

    impl Fmt for Struct {
        fn fmt(&self) -> &(impl crate::write_to::WriteTo + ?Sized) {
            fmt!({ s: Struct = self } => "a = " {s.a} ", b = " {s.b})
        }
    }

    impl FmtDebug for Struct {
        fn fmt_debug(&self) -> &(impl crate::write_to::WriteTo + ?Sized) {
            fmt_struct!({ _self: Struct = self } => Struct; { a: {_self.a}, b: {_self.b} })
        }
    }

    struct Tuple(i32, bool);

    struct Struct2 {
        a2: i32,
        b2: bool,
    }

    impl WriteTo for Struct2 {
        fmt! { [s] => {s.a2} "sussy rizz" {s.b2} }
    }

    let struct_ = Struct { a: 12, b: true };
    let s = fmt_struct!({} => Struct; { a: {@a=struct_.a}, b: {@b=struct_.b} });
    let s0 = s.to_string();
    assert_eq!(s0, "Struct { a: 12, b: true }");

    let s = Struct2 { a2: 123, b2: false };
    let s0 = s.to_string();
    assert_eq!(s0, "123sussy rizzfalse");

    let tuple = Tuple(99, true);
    let s = fmt_tuple_struct!({} => Tuple; ({@a=tuple.0}, {@b=tuple.1}));
    let s0 = s.to_string();
    assert_eq!(s0, "Tuple(99, true)");

    const S: &str = fmt_struct!({} => Struct; { a: {@a:i32=234}, b: {@b=false} });
    let s0 = ToString::to_string(S);
    assert_eq!(s0, "Struct { a: 234, b: false }");

    const S1: &str = fmt_tuple_struct!({} => Tuple; ({@a=234}, {@b=false}));
    assert_eq!(S1, "Tuple(234, false)");

    macro_rules! xyz {
        () => {
            "XYZ"
        };
    }

    let a = "abc";
    let b = "def";
    let s = fmt!({} => "123" @[xyz!()] "abc" {@a} {@b} "abc" ln [""]);
    let s0 = ToString::to_string(s);
    assert_eq!(s0.len(), s.len_hint());
    assert!(ends_in_newline(s));
    assert_eq!(s0, "123XYZabcabcdefabc\n");

    let a = &mut *String::from("abc");
    let s = fmt!({} => "123" @[xyz!()] "abc" {@a} "abc");
    let s0 = ToString::to_string(s);
    assert_eq!(s0.len(), s.len_hint());
    assert_eq!(s0, "123XYZabcabcabc");

    let a = String::from("abc");
    let s = fmt!({} => "123" @[xyz!()] "abc" {@a} "abc");
    let s0 = ToString::to_string(s);
    assert_eq!(s0.len(), s.len_hint());
    assert_eq!(s0, "123XYZabcabcabc");

    let a = &String::from("abc");
    let s = fmt!({} => "123" @[xyz!()] "abc" {@a} "abc");
    let s0 = ToString::to_string(s);
    assert_eq!(s0.len(), s.len_hint());
    assert_eq!(s0, "123XYZabcabcabc");

    let a = &mut String::from("abc");
    let s = fmt!({} => "123" @[xyz!()] "abc" {@a} "abc");
    let s0 = ToString::to_string(s);
    assert_eq!(s0.len(), s.len_hint());
    assert_eq!(s0, "123XYZabcabcabc");

    let a = &mut String::from("abc");
    let s = fmt!({noref} => "123" @[xyz!()] "abc" {@a} "abc");
    let s0 = ToString::to_string(&s);
    assert_eq!(s0.len(), s.len_hint());
    assert_eq!(s0, "123XYZabcabcabc");

    let a: Box<str> = Box::from("abc");
    let s = fmt!({} => "123" @[xyz!()] "abc" {@a} "abc");
    let s0 = ToString::to_string(s);
    assert_eq!(s0.len(), s.len_hint());
    assert_eq!(s0, "123XYZabcabcabc");

    let a = &3_i32;
    let s = fmt!({} => "123" @[xyz!()] "abc" {@a} "abc");
    let s0 = ToString::to_string(s);
    assert_eq!(s0, "123XYZabc3abc");

    let a = 3_i32;
    let w = fmt!({} => "123" @[xyz!()] "abc" {@a} "abc");
    let s0 = ToString::to_string(w);
    assert_eq!(s0, "123XYZabc3abc");

    let a = 3_i32;
    let w = fmt!({} => "123" @[xyz!()] "abc" {@a} "abc");
    // let w = fmt!({} => "123" @[xyz!()] "abc" {@a:i32} "abc");
    let s0 = ToString::to_string(w);
    assert_eq!(s0, "123XYZabc3abc");

    const _S: &str = fmt!({} => "123" @[xyz!()] "abc" "abc" 123);
    assert_eq!(_S, "123XYZabcabc123");

    let a = 3_i32;
    const I: i32 = 32;
    let s = fmt!({} => "123" @[xyz!()] "abc" {@a} "123" {I} "abc");
    let s0 = ToString::to_string(s);

    let a = 3_i32;
    let s = fmt!({} => "123" @[xyz!()] "abc" {@a;?} "123" {I;?} "abc");
    let s0 = ToString::to_string(s);

    let a = 3_i32;
    let s = fmt!({} => "123" @[xyz!()] "abc" {@a;h} "123" {I;b} "abc");
    let s0 = ToString::to_string(s);

    let a = 12.1234_f32;
    const F: f32 = 12.1234;
    let s = fmt!({} => "999" @[xyz!()] "abc" {@a;std .3} "abc" {F;} "abc");
    let s0 = ToString::to_string(s);
    // assert_eq!(s0, "999XYZabc12.123abc12.12abc");

    fn const_fn(a: i32, b: i32) -> i32 {
        a + b
    }
    let s = fmt!({} => "123" @[xyz!()] "abc" {I} "abc" {@d=456});
    let s0 = ToString::to_string(s);

    let s = fmt!({} => "123" @[xyz!()] "abc" {const_fn(1, 2)} "abc");
    // let s = fmt!({ & } => "123" @[xyz!()] "abc" (&const_fn(1, 2) ) "abc");
    let s0 = ToString::to_string(s);
}

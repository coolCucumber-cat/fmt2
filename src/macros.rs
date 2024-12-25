#[macro_export]
macro_rules! noop {
    ($($tt:tt)*) => {};
}

#[macro_export]
macro_rules! get_write_to_from_fmt_args {
    { $value:expr; { $($fmt_args:tt)* } } => {
        $crate::get_write_to_from_fmt_args! { $value; $($fmt_args)* }
    };
	{ $value:expr; noderef } => {
		$value
	};
    { $value:expr; trait $tr:path } => {{
        use $tr as _;
		$value.fmt_internal()
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
}

#[macro_export]
macro_rules! write_fmt_single_internal {
	($writer:expr => { $value:expr; $($fmt_args:tt)* } => $handle_result:path $(=> $label:lifetime)?) => {{
		use $crate::write::Write as _;
		$handle_result!(
			($writer).write_advanced::<_, false, false>(
				$crate::get_write_to_from_fmt_args! { $value; $($fmt_args)* }
			)
			$(, $label)?
		);
	}};

	($writer:expr => [$("", )*] => $handle_result:path $(=> $label:lifetime)?) => {{
		compile_error!("unreachable. dev error or bug using macro");
	}};

	($writer:expr => [$($value:expr, )+] => $handle_result:path $(=> $label:lifetime)?) => {{
		use $crate::write::Write as _;
		const S: &str = ::core::concat!($($value),+);
		if S.len() != 0 {
			$handle_result!($writer.write_str(S) $(, $label)?);
		}
	}};
}

#[macro_export]
macro_rules! try_internal {
    ($value:expr $(, $label:lifetime)?) => {
        ($value)?;
    };
}
#[macro_export]
macro_rules! write_fmt_single_try_internal {
    ($writer:expr => $tt:tt) => {
		$crate::write_fmt_single_internal!($writer => $tt => $crate::try_internal);
	};
}

#[macro_export]
macro_rules! return_on_err_internal {
    ($value:expr $(, $label:lifetime)?) => {
        if let ::core::result::Result::Err(err) = $value {
            return ::core::result::Result::Err(err);
        }
    };
}
#[macro_export]
macro_rules! write_fmt_single_return_internal {
    ($writer:expr => $tt:tt) => {
		$crate::write_fmt_single_internal!($writer => $tt => $crate::return_on_err_internal);
	};
}

#[macro_export]
macro_rules! return_tuple_on_err_internal {
    ($value:expr $(, $label:lifetime)?) => {
        if let ::core::result::Result::Err(err) = $value {
            return;
        }
    };
}
#[macro_export]
macro_rules! write_fmt_single_return_infallible_internal {
    ($writer:expr => $tt:tt) => {
		$crate::write_fmt_single_internal!($writer => $tt => $crate::return_tuple_on_err_internal);
	};
}

#[macro_export]
macro_rules! break_on_err_internal {
    ($value:expr, $label:lifetime) => {
        if let ::core::result::Result::Err(err) = $value {
            break $label::core::result::Result::Err(err);
        }
    };
}
#[macro_export]
macro_rules! write_fmt_single_break_internal {
    ($label:lifetime $writer:expr => $tt:tt) => {
		$crate::write_fmt_single_internal!($writer => $tt => $crate::break_on_err_internal => $label);
	};
}

#[macro_export]
macro_rules! break_tuple_on_err_internal {
    ($value:expr, $label:lifetime) => {
        if let ::core::result::Result::Err(err) = $value {
            break $label;
        }
    };
}
#[macro_export]
macro_rules! write_fmt_single_break_infallible_internal {
    ($label:lifetime $writer:expr => $tt:tt) => {
		$crate::write_fmt_single_internal!($writer => $tt => $crate::break_tuple_on_err_internal => $label);
	};
}

#[macro_export]
macro_rules! len_hint_fmt_single_internal {
	({ $value:expr; $($fmt_args:tt)* }) => {
		$crate::write_to::WriteTo::len_hint(
			$crate::get_write_to_from_fmt_args! { $value; $($fmt_args)* }
		)
	};

	([$("", )*]) => {{
		compile_error!("unreachable. dev error or bug using macro");
	}};

	([$($value:expr, )+]) => {
		::core::concat!($($value),+).len()
	};
}

#[macro_export]
macro_rules! fn_write_to_internal {
	($writer:expr => $($tt:tt)*) => {
		$(
			$crate::write_fmt_single_return_internal!($writer => $tt);
		)*
	};
}

#[macro_export]
macro_rules! fn_len_hint_internal {
	($($tt:tt)*) => {
		0
		$(
			+ $crate::len_hint_fmt_single_internal!($tt)
		)*
	};
}

#[macro_export]
macro_rules! impl_for_write_to_internal {
	{ $(internal $internal0:tt,)* $(external { $field_name:ident; $fmt_args:tt }, $(internal $internal:tt,)*)* } => {
		#[inline]
		fn write_to<W>(&self, w: &mut W) -> Result<(), W::Error>
			where
				W: $crate::write::Write + ?Sized {

			$crate::fn_write_to_internal!(
				w =>
				$($internal0)*
				$(
					{ self.$field_name; $fmt_args }
					$($internal)*
				)*
			);
			::core::result::Result::Ok(())
		}

		#[inline]
		fn len_hint(&self) -> usize {
			$crate::fn_len_hint_internal!(
				$($internal0)*
				$(
					{ self.$field_name; $fmt_args }
					$($internal)*
				)*
			)
		}
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! fmt_internal {
	// do recursion

	// literal
	{
		input: { $([$($prev:expr),* $(,)?], )* $literal:literal, $($inputs:tt, )* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal! {
			input: { [$($($prev, )*)* $literal], $($inputs, )* },
			output: { $($outputs)* },
			args: $args
		}
	};
	// ln
	{
		input: { $([$($prev:expr),* $(,)?], )* ln, $($inputs:tt, )* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal! {
			input: { [$($($prev, )*)* "\n"], $($inputs, )* },
			output: { $($outputs)* },
			args: $args
		}
	};
	// literal in a non-capturing expression (make it literal)
	{
		input: { $([$($prev:expr),* $(,)?], )* ($literal:literal $(;)?), $($inputs:tt, )* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal! {
			input: { [$($($prev, )*)* $literal], $($inputs, )* },
			output: { $($outputs)* },
			args: $args
		}
	};
	// literal in a capturing expression (make it literal)
	{
		input: { $([$($prev:expr),* $(,)?], )* { $field_name:ident $(: $ty:ty)? = $literal:literal $(;)? }, $($inputs:tt, )* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal! {
			input: { [$($($prev, )*)* $literal], $($inputs, )* },
			output: { $($outputs)* },
			args: $args
		}
	};
	// empty group of literals
	{
		input: { [$(""),* $(,)?], $($inputs:tt, )* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal! {
			input: { $($inputs, )* },
			output: { $($outputs)* },
			args: $args
		}
	};
	// 2 groups of literals
	{
		input: { [$($literal1:expr),* $(,)?], [$($literal2:expr),* $(,)?], $($inputs:tt, )* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal! {
			input: { [$($literal1, )* $($literal2),*], $($inputs, )* },
			output: { $($outputs)* },
			args: $args
		}
	};
	// literals
	{
		input: { [$($literal:expr),* $(,)?], $($inputs:tt, )* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal! {
			input: { $($inputs, )* },
			output: { $($outputs)* internal [$($literal, )*], },
			args: $args
		}
	};

	// non-capturing expression
	{
		input: { ($value:expr $(; $($fmt_args:tt)*)?), $($inputs:tt, )* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal! {
			input: { $($inputs, )* },
			output: { $($outputs)* internal { $value; $($($fmt_args)*)? }, },
			args: $args
		}
	};

	// capturing expression with generic type
	{
		input: { { $field_name:ident = $value:expr $(; $($fmt_args:tt)*)? }, $($inputs:tt, )* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal! {
			input: { $($inputs, )* },
			output: { $($outputs)*
				external { $field_name, generic: $field_name, ty: , = $value; { $($($fmt_args)*)? } },
			},
			args: $args
		}
	};
	// capturing expression with concrete type
	{
		input: { { $field_name:ident : $ty:ty = $value:expr $(; $($fmt_args:tt)*)? }, $($inputs:tt, )* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal! {
			input: { $($inputs, )* },
			output: { $($outputs)*
				external { $field_name, generic: , ty: $ty, = $value; { $($($fmt_args)*)? } },
			},
			args: $args
		}
	};
	// capturing expression using variable as name and as value
	{
		input: { { $field_name:ident $(: $ty:ty)? $(; $($fmt_args:tt)*)? }, $($inputs:tt, )* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal! {
			input: { { $field_name $(: $ty)? = $field_name; $($($fmt_args)*)? }, $($inputs, )* },
			output: { $($outputs)* },
			args: $args
		}
	};

	// error (macros must be in square brackets)
	{
		input: { $name:ident!$tt:tt, $($inputs:tt, )* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {{
		compile_error!(concat!(
			"macros must be in [square brackets]\n",
			stringify!($name), "!", stringify!($tt),
		));
	}};
	// error
	{
		input: { $tt:tt, $($inputs:tt, )* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {{
		compile_error!(concat!(
			"expressions must be either valid literals or in (round), {curly} or [square] brackets\n",
			"see documentation for the `fmt` macro\n",
			stringify!($tt),
		));
	}};

	// terminate recursion

	// nothing (empty string)
	{
		input: {},
		output: {},
		args: $args:tt
	} => {
		""
	};
	// error
	{
		input: {},
		output: { $(internal [$("", )*], )* },
		args: $args:tt
	} => {{
		compile_error!("unreachable. dev error in macro");
	}};
	// only literals (concat)
	{
		input: {},
		output: { internal [$($literals:expr, )+], },
		args: $args:tt
	} => {
		::core::concat!($($literals),*)
	};
	// error
	{
		input: {},
		output: { $(internal [$($literals:expr, )*], )* },
		args: $args:tt
	} => {{
		compile_error!("unreachable. dev error in macro");
	}};

	// only one non-capturing, WriteTo expression (no wrapper struct, since it's only one and already WriteTo)
	{
		input: {},
		output: { internal { $value:expr; } },
		args: $args:tt
	} => {{
		use $crate::write_to::Fmt as _;
		($value).fmt()
	}};
	// only one capturing, WriteTo expression (no wrapper struct, since it's only one and already WriteTo)
	{
		input: {},
		output: { external { $field_name:ident = $value:expr; } },
		args: $args:tt
	} => {{
		use $crate::write_to::Fmt as _;
		($value).fmt()
	}};

	// only one capturing, WriteTo expression (no wrapper struct, since it's only one and already WriteTo. no borrow it's a concrete type)
	{
		input: {},
		output: { external { $field_name:ident : $ty:ty = $value:expr; } },
		args: $args:tt
	} => {
		$value
	};

	// at least one non-capturing expression, no capturing expressions, any amount of literals (but not one non-capturing, WriteTo expression because it's already covered)
// 	{
// 		input: {},
// 		output: { $(internal $internal:tt,)+ },
// 		args: {
// 			lifetime: $lifetime:lifetime,
// 			optional_lifetime: $($optional_lifetime:lifetime)?,
// 			reference: { $($reference:tt)? },
// 		}
// 	} => {$($reference)? {
// 		struct W;
//
// 		impl $crate::write_to::WriteTo for W {
// 			$crate::impl_for_write_to_internal! { $(internal $internal,)+ }
// 		}
//
// 		W
// 	}};

	// combination of sources (excluding ones that are already covered above) and where the capturing values all have concrete types
	{
		input: {},
		output: { $(internal $internal0:tt,)* $(external { $field_name:ident, generic: , ty: $ty:ty, = $value:expr; $fmt_args:tt }, $(internal $internal:tt,)*)* },
		args: {
			lifetime: $lifetime:lifetime,
			optional_lifetime: $($optional_lifetime:lifetime)?,
			reference: { $($reference:tt)? },
		}
	} => {$($reference)? {
		#[allow(non_camel_case_types)]
		struct W<$($optional_lifetime)?> {
			$($field_name : $ty),*
		}

		#[allow(non_camel_case_types)]
		impl<$($optional_lifetime)?> $crate::write_to::WriteTo for W<$($optional_lifetime)?> {
			$crate::impl_for_write_to_internal! {
				$(internal $internal0,)*
				$(external { $field_name; $fmt_args }, $(internal $internal,)*)*
			}
		}

		W {
			$($field_name : $value),*
		}
	}};
	// combination of sources (excluding ones that are already covered above)
	{
		input: {},
		output: {
			$(internal $internal0:tt,)*
			$(
				external {
					$field_name:ident,
					generic: $($generic:ident)?,
					ty: $($ty:ty)?,
					= $value:expr;
					$fmt_args:tt
				},
				$(internal $internal:tt,)*
			)*
		},
		args: {
			lifetime: $lifetime:lifetime,
			optional_lifetime: $($optional_lifetime:lifetime)?,
			reference: { $($reference:tt)? },
		}
	} => {$($reference)? {
		// for syntax highlighting
		#[allow(unused)]
		{
			$(let $field_name: ();)*
		}

		#[allow(non_camel_case_types)]
		struct W<$lifetime, $($($generic : $crate::write_to::WriteTo + ?Sized, )?)+> {
			$($field_name : $(&$lifetime $generic)? $($ty)? ),*
		}

		#[allow(non_camel_case_types)]
		impl<$lifetime, $($($generic : $crate::write_to::WriteTo + ?Sized, )?)+> $crate::write_to::WriteTo for W<$lifetime, $($($generic, )?)+> {
			$crate::impl_for_write_to_internal! {
				$(internal $internal0,)*
				$(external { $field_name; noderef }, $(internal $internal,)*)+
			}
		}

		W {
			$($field_name :
				$({
					$crate::noop!($generic);
					$crate::get_write_to_from_fmt_args! { $value; $fmt_args }
				})?
				$({
					$crate::noop!($ty);
					$value
				})?
			),*
		}
	}};
}

#[macro_export]
#[doc(hidden)]
macro_rules! write_fmt_internal {
	// do recursion

	// literal
	{
		input: { $([$($prev:expr),* $(,)?], )* $literal:literal, $($inputs:tt, )* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::write_fmt_internal! {
			input: { [$($($prev, )*)* $literal], $($inputs, )* },
			output: { $($outputs)* },
			args: $args
		}
	};
	// ln
	{
		input: { $([$($prev:expr),* $(,)?], )* ln, $($inputs:tt, )* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::write_fmt_internal! {
			input: { [$($($prev, )*)* "\n"], $($inputs, )* },
			output: { $($outputs)* },
			args: $args
		}
	};
	// literal in an expression (make it literal)
	{
		input: { $([$($prev:expr),* $(,)?], )* { $literal:literal $(;)? }, $($inputs:tt, )* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::write_fmt_internal! {
			input: { [$($($prev, )*)* $literal], $($inputs, )* },
			output: { $($outputs)* },
			args: $args
		}
	};
	// empty group of literals
	{
		input: { [$(""),* $(,)?], $($inputs:tt, )* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::write_fmt_internal! {
			input: { $($inputs, )* },
			output: { $($outputs)* },
			args: $args
		}
	};
	// 2 groups of literals
	{
		input: { [$($literal1:expr),* $(,)?], [$($literal2:expr),* $(,)?], $($inputs:tt, )* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::write_fmt_internal! {
			input: { [$($literal1, )* $($literal2),*], $($inputs, )* },
			output: { $($outputs)* },
			args: $args
		}
	};
	// literals
	{
		input: { [$($literal:expr),* $(,)?], $($inputs:tt, )* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::write_fmt_internal! {
			input: { $($inputs, )* },
			output: { $($outputs)* [$($literal, )*], },
			args: $args
		}
	};
	// expression
	{
		input: { { $value:expr $(; $($fmt_args:tt)*)? }, $($inputs:tt, )* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::write_fmt_internal! {
			input: { $($inputs, )* },
			output: { $($outputs)*
				{ $value; $($($fmt_args)*)? },
			},
			args: $args
		}
	};

	// error (macros must be in square brackets)
	{
		input: { $name:ident!$tt:tt, $($inputs:tt, )* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {{
		compile_error!(concat!(
			"macros must be in [square brackets]\n",
			stringify!($name), "!", stringify!($tt),
		));
	}};
	// error
	{
		input: { $tt:tt, $($inputs:tt, )* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {{
		compile_error!(concat!(
			"expressions must be either valid literals or in {curly} or [square] brackets\n",
			"see documentation for the `write_fmt` macro\n",
			stringify!($tt),
		));
	}};

	// terminate recursion
	{
		input: {},
		output: { $($fmt:tt,)* },
		args: {
			writer: $writer:expr,
			ignore_err: false,
		}
	} => {'block: {
		$(
			$crate::write_fmt_single_break_internal!('block $writer => $fmt);
		)*
		::core::result::Result::Ok(())
	}};
	{
		input: {},
		output: { $($fmt:tt,)* },
		args: {
			writer: $writer:expr,
			ignore_err: true,
		}
	} => {'block: {
		$(
			$crate::write_fmt_single_break_infallible_internal!('block $writer => $fmt);
		)*
	}};
}

#[macro_export]
macro_rules! fmt_advanced {
	{ { & } => $($tt:tt)* } => {
		$crate::fmt_internal! {
			input: { $($tt, )* },
			output: {},
			args: {
				lifetime: 'a,
				optional_lifetime:,
				reference: { & },
			}
		}
	};
	{ {} => $($tt:tt)* } => {
		$crate::fmt_internal! {
			input: { $($tt, )* },
			output: {},
			args: {
				lifetime: 'a,
				optional_lifetime:,
				reference: {},
			}
		}
	};
	{ { &$name:ident : $ty:ty } => $($tt:tt)* } => {
		$crate::fmt_advanced! { { &$name : $ty = $name} => $($tt)* }
	};
	{ { &self : $ty:ty = $value:expr } => $($tt:tt)* } => {{
		$crate::fmt_advanced! { { &_self : $ty = $value } => $($tt)* }
	}};
	{ { &$name:ident : $ty:ty = $value:expr } => $($tt:tt)* } => {{
		struct W($ty);

		impl W {
			fn new(t: &$ty) -> &Self {
				unsafe { &*(t as *const $ty as *const Self) }
			}
		}

		impl $crate::write_to::WriteTo for W {
			fn write_to<W>(&self, writer: &mut W) -> Result<(), W::Error>
			where
				W: $crate::write::Write + ?Sized,
			{
				let $name: &$ty = &self.0;
				fmt_write!(? writer => $($tt)*)
			}
		}

		W::new($value)
	}};
	{ (? #) => $($tt:tt)* } => {
		$crate::write_fmt_internal! {
			input: { $($tt, )* },
			output: {},
			args: {
				writer: (::std::io::stdout()),
				ignore_err: false,
			}
		}
	};
	{ (#) => $($tt:tt)* } => {
		$crate::write_fmt_internal! {
			input: { $($tt, )* },
			output: {},
			args: {
				writer: (::std::io::stdout()),
				ignore_err: true,
			}
		}
	};
	{ (? $writer:expr) => $($tt:tt)* } => {
		$crate::write_fmt_internal! {
			input: { $($tt, )* },
			output: {},
			args: {
				writer: $writer,
				ignore_err: false,
			}
		}
	};
	{ ($writer:expr) => $($tt:tt)* } => {
		$crate::write_fmt_internal! {
			input: { $($tt, )* },
			output: {},
			args: {
				writer: $writer,
				ignore_err: true,
			}
		}
	};
}

#[macro_export]
macro_rules! fmt {
	($($tt:tt)*) => {
		$crate::fmt_advanced!({ & } => $($tt)*)
	};
}

#[macro_export]
macro_rules! fmt_write {
	(# => $($tt:tt)*) => {
		$crate::fmt_advanced!((#) => $($tt)*)
	};
	(? # => $($tt:tt)*) => {
		$crate::fmt_advanced!((? #) => $($tt)*)
	};
	($writer:expr => $($tt:tt)*) => {
		$crate::fmt_advanced!(($writer) => $($tt)*)
	};
	(? $writer:expr => $($tt:tt)*) => {
		$crate::fmt_advanced!((? $writer) => $($tt)*)
	};
}

#[macro_export]
macro_rules! fmt_struct {
	($fmt:tt => $name:ident; { $field0:ident : $tt0:tt $(, $field:ident : $tt:tt)* $(,)? }) => {
		$crate::fmt_advanced! { $fmt => [stringify!($name)] " { " [stringify!($field0)] ": " $tt0 $(", " [stringify!($field)] ": " $tt)* " }" }
	};

	($fmt:tt => $name:ident; { $field0:ident $(, $field:ident)* $(,)? }) => {
		$crate::fmt_advanced! { $fmt => [stringify!($name)] " { " [stringify!($field0)] ": " {$field0} $(", " [stringify!($field)] ": " {$field})* " }" }
	};

	($fmt:tt => $name:ident; {}) => {
		$crate::fmt_advanced! { $fmt => [stringify!($name)] " {}" }
	};
}

#[macro_export]
macro_rules! fmt_struct_debug {
	($fmt:tt => $name:ident; { $field0:ident $(, $field:ident)* $(,)? }) => {
		$crate::fmt_advanced! { $fmt => [stringify!($name)] " { " [stringify!($field0)] ": " {$field0;?} $(", " [stringify!($field)] ": " {$field;?})* " }" }
	};

	($fmt:tt => $name:ident; {}) => {
		$crate::fmt_advanced! { $fmt => [stringify!($name)] " {}" }
	};
}

#[macro_export]
macro_rules! fmt_tuple_struct {
	($fmt:tt => $($name:ident;)? ($tt0:tt $(, $tt:tt)* $(,)?) ) => {
		$crate::fmt_advanced! { $fmt => $([stringify!($name)])? "(" $tt0 $(", " $tt)* ")" }
	};

	($fmt:tt => $($name:ident;)? () ) => {
		$crate::fmt_advanced! { $fmt => $([stringify!($name)])? "()" }
	};
}

#[macro_export]
macro_rules! fmt_unit_struct {
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
    use core::ops::Deref;

    use crate::{
        write::Write,
        write_to::{Fmt, FmtDebug, ToString, WriteTo},
    };

    struct Struct {
        a: i32,
        b: bool,
    }

    impl Fmt for Struct {
        fn fmt(&self) -> &(impl crate::write_to::WriteTo + ?Sized) {
            fmt_advanced!({ &s: Struct = self} => "a = " {s.a} ", b = " {s.b})
        }
    }

    impl FmtDebug for Struct {
        fn fmt_debug(&self) -> &(impl crate::write_to::WriteTo + ?Sized) {
            fmt_struct!({ &s: Struct = self } => Struct; { a: {s.a}, b: {s.b} })
        }
    }

    struct Tuple(i32, bool);

    let struct_ = Struct { a: 12, b: true };
    let s = fmt_struct!({ & } => Struct; { a: {a = struct_.a}, b: {b = struct_.b} });
    let s0 = s.to_string();
    assert_eq!(s0, "Struct { a: 12, b: true }");

    let tuple = Tuple(99, true);
    let s = fmt_tuple_struct!({ & } => Tuple; ({a = tuple.0}, {b = tuple.1}));
    let s0 = s.to_string();
    assert_eq!(s0, "Tuple(99, true)");

    const S: &str = fmt_struct!({ & } => Struct; { a: {a: i32 = 234}, b: {b = false} });
    let s0 = ToString::to_string(S);
    assert_eq!(s0, "Struct { a: 234, b: false }");

    const S1: &str = fmt_tuple_struct!({ & } => Tuple; ({a = 234}, {b = false}));
    assert_eq!(S1, "Tuple(234, false)");

    macro_rules! xyz {
        () => {
            "XYZ"
        };
    }

    let a = "abc";
    let s = fmt!("123" [xyz!()] "abc" {a} "abc");
    let s0 = ToString::to_string(s);
    assert_eq!(s0.len(), s.len_hint());
    assert_eq!(s0, "123XYZabcabcabc");

    let a = &mut *String::from("abc");
    let s = fmt!("123" [xyz!()] "abc" {a} "abc");
    let s0 = ToString::to_string(s);
    assert_eq!(s0.len(), s.len_hint());
    assert_eq!(s0, "123XYZabcabcabc");

    let a = String::from("abc");
    let s = fmt!("123" [xyz!()] "abc" {a} "abc");
    let s0 = ToString::to_string(s);
    assert_eq!(s0.len(), s.len_hint());
    assert_eq!(s0, "123XYZabcabcabc");

    let a = &String::from("abc");
    let s = fmt!("123" [xyz!()] "abc" {a} "abc");
    let s0 = ToString::to_string(s);
    assert_eq!(s0.len(), s.len_hint());
    assert_eq!(s0, "123XYZabcabcabc");

    let a = &mut String::from("abc");
    let s = fmt!("123" [xyz!()] "abc" {a} "abc");
    let s0 = ToString::to_string(s);
    assert_eq!(s0.len(), s.len_hint());
    assert_eq!(s0, "123XYZabcabcabc");

    let a = &mut String::from("abc");
    let s = fmt_advanced!({} => "123" [xyz!()] "abc" {a} "abc");
    let s0 = ToString::to_string(&s);
    assert_eq!(s0.len(), s.len_hint());
    assert_eq!(s0, "123XYZabcabcabc");

    let a: Box<str> = Box::from("abc");
    let s = fmt!("123" [xyz!()] "abc" {a} "abc");
    let s0 = ToString::to_string(s);
    assert_eq!(s0.len(), s.len_hint());
    assert_eq!(s0, "123XYZabcabcabc");

    let a = &3_i32;
    let s = fmt!("123" [xyz!()] "abc" {a} "abc");
    let s0 = ToString::to_string(s);
    assert_eq!(s0, "123XYZabc3abc");

    let a = 3_i32;
    let w = fmt!("123" [xyz!()] "abc" {a} "abc");
    let s0 = ToString::to_string(w);
    assert_eq!(s0, "123XYZabc3abc");

    let a = 3_i32;
    let w = fmt!("123" [xyz!()] "abc" {a: i32} "abc");
    let s0 = ToString::to_string(w);
    assert_eq!(s0, "123XYZabc3abc");

    const _S: &str = fmt!("123" [xyz!()] "abc" "abc" 123);
    assert_eq!(_S, "123XYZabcabc123");

    let a = 3_i32;
    const I: i32 = 32;
    let s = fmt!("123" [xyz!()] "abc" {a} "123" (I) "abc");
    let s0 = ToString::to_string(s);

    let a = 3_i32;
    let s = fmt!("123" [xyz!()] "abc" {a;?} "123" (I;?) "abc");
    let s0 = ToString::to_string(s);

    let a = 3_i32;
    let s = fmt!("123" [xyz!()] "abc" {a;h} "123" (I;b) "abc");
    let s0 = ToString::to_string(s);

    let a = 12.1234_f32;
    const F: f32 = 12.1234;
    let s = fmt!("999" [xyz!()] "abc" {a;} "abc" (F;) "abc");
    let s0 = ToString::to_string(s);
    // assert_eq!(s0, "999XYZabc12.123abc12.12abc");

    fn const_fn(a: i32, b: i32) -> i32 {
        a + b
    }
    let s = fmt!("123" [xyz!()] "abc" (&I) "abc" {d = 456});
    let s0 = ToString::to_string(s);

    let s = fmt!("123" [xyz!()] "abc" (&const_fn(1, 2) ) "abc");
    // let s = fmt!("123" [xyz!()] "abc" (&const_fn(1, 2) ) "abc");
    let s0 = ToString::to_string(s);
}

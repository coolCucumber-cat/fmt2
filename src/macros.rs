#[macro_export]
macro_rules! noop {
    ($($tt:tt)*) => {};
}

#[macro_export]
macro_rules! use_internal_writable_trait_from_fmt_args {
    ($vis:vis use { { $($fmt_args:tt)* } } as $name:tt;) => {
        $crate::use_internal_writable_trait_from_fmt_args!($vis use { $($fmt_args)* } as $name;);
    };
    ($vis:vis use { trait $tr:path } as $name:tt;) => {
        $vis use $tr as $name;
    };
    ($vis:vis use {} as $name:tt;) => {
        $vis use $crate::writable::WritableInternal as $name;
    };
    ($vis:vis use { ? } as $name:tt;) => {
        $vis use $crate::writable::WritableDebugInternal as $name;
    };
    ($vis:vis use { b } as $name:tt;) => {
        $vis use $crate::writable::WritableBinaryInternal as $name;
    };
    ($vis:vis use { h } as $name:tt;) => {
        $vis use $crate::writable::WritableHexadecimalInternal as $name;
    };
}

#[macro_export]
macro_rules! write_fmt_single_try_internal {
	($writer:expr => { $value:expr; $($fmt_args:tt)* }) => {{
		$crate::use_internal_writable_trait_from_fmt_args!(use { $($fmt_args)* } as _;);
		($value).write_to_internal($writer)?;
	}};

	($writer:expr => [$("", )*]) => {{
		compile_error!("unreachable. dev error or bug using macro");
	}};

	($writer:expr => [$($value:expr, )+]) => {{
		use $crate::write::Write as _;
		const S: &str = ::core::concat!($($value),+);
		if S.len() != 0 {
			$writer.write_str(S)?;
		}
	}};
}

#[macro_export]
macro_rules! write_fmt_single_return_internal {
	($writer:expr => { $value:expr; $($fmt_args:tt)* }) => {{
		$crate::use_internal_writable_trait_from_fmt_args!(use { $($fmt_args)* } as _;);
		if let ::core::result::Result::Err(err) = ($value).write_to_internal($writer) {
			return ::core::result::Result::Err(err);
		}
	}};

	($writer:expr => [$("", )*]) => {{
		compile_error!("unreachable. dev error or bug using macro");
	}};

	($writer:expr => [$($value:expr, )+]) => {{
		use $crate::write::Write as _;
		const S: &str = ::core::concat!($($value),+);
		if S.len() != 0 {
			if let ::core::result::Result::Err(err) = $writer.write_str(S) {
				return ::core::result::Result::Err(err);
			}
		}
	}};
}

#[macro_export]
macro_rules! write_fmt_single_break_internal {
	($block_label:lifetime $writer:expr => { $value:expr; $($fmt_args:tt)* }) => {{
		$crate::use_internal_writable_trait_from_fmt_args!(use { $($fmt_args)* } as _;);
		if let ::core::result::Result::Err(err) = ($value).write_to_internal($writer) {
			break $block_label ::core::result::Result::Err(err);
		}
	}};

	($block_label:lifetime $writer:expr => [$("", )*]) => {{
		compile_error!("unreachable. dev error or bug using macro");
	}};

	($block_label:lifetime $writer:expr => [$($value:expr, )+]) => {{
		use $crate::write::Write as _;
		const S: &str = ::core::concat!($($value),+);
		if S.len() != 0 {
			if let ::core::result::Result::Err(err) = $writer.write_str(S) {
				break $block_label ::core::result::Result::Err(err);
			}
		}
	}};
}

#[macro_export]
macro_rules! write_fmt_single_break_ignore_err_internal {
	($block_label:lifetime $writer:expr => { $value:expr; $($fmt_args:tt)* }) => {{
		$crate::use_internal_writable_trait_from_fmt_args!(use { $($fmt_args)* } as _;);
		if let ::core::result::Result::Err(_) = ($value).write_to_internal($writer) {
			break $block_label;
		}
	}};

	($block_label:lifetime $writer:expr => [$("", )*]) => {{
		compile_error!("unreachable. dev error or bug using macro");
	}};

	($block_label:lifetime $writer:expr => [$($value:expr, )+]) => {{
		use $crate::write::Write as _;
		const S: &str = ::core::concat!($($value),+);
		if S.len() != 0 {
			if let ::core::result::Result::Err(_) = $writer.write_str(S) {
				break $block_label;
			}
		}
	}};
}

#[macro_export]
macro_rules! len_hint_fmt_single_internal {
	({ $value:expr; $($fmt_args:tt)* }) => {{
		$crate::use_internal_writable_trait_from_fmt_args!(use { $($fmt_args)* } as _;);
		($value).len_hint_internal()
	}};

	([$("", )*]) => {{
		compile_error!("unreachable. dev error or bug using macro");
	}};

	([$($value:expr, )+]) => {{
		const S: &str = ::core::concat!($($value),+);
		S.len()
	}};
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
macro_rules! impl_for_writable_with_fn_name_internal {
	{ $fn_name:ident : $(internal $internal0:tt,)* $(external { $field_name:ident; $fmt_args:tt }, $(internal $internal:tt,)*)* } => {
		#[inline]
		fn $fn_name<W>(&self, w: &mut W) -> Result<(), W::Error>
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

#[macro_export]
macro_rules! impl_for_writable_internal {
	($($tt:tt)*) => {
		$crate::impl_for_writable_with_fn_name_internal!(write_to: $($tt)*);
	};
}

#[macro_export]
macro_rules! impl_for_writable_debug_internal {
	($($tt:tt)*) => {
		$crate::impl_for_writable_with_fn_name_internal!(write_to_debug: $($tt)*);
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

	// declare the functions needed for the trait, nothing else
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
			writable_fn_name: $writable_fn_name:ident,
		}
	} => {
		$crate::impl_for_writable_with_fn_name_internal! {
			$writable_fn_name:
			$(internal $internal0,)*
			$(external { $field_name; $fmt_args }, $(internal $internal,)*)*
		}
	};

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

	// only one non-capturing, writable expression (no wrapper struct, since it's only one and already writable)
	{
		input: {},
		output: { internal { $value:expr; } },
		args: $args:tt
	} => {{
		use $crate::writable::WritableInternal as _;
		($value).borrow_writable_internal()
	}};
	// only one capturing, writable expression (no wrapper struct, since it's only one and already writable)
	{
		input: {},
		output: { external { $field_name:ident = $value:expr; } },
		args: $args:tt
	} => {{
		use $crate::writable::WritableInternal as _;
		($value).borrow_writable_internal()
	}};

	// only one capturing, writable expression (no wrapper struct, since it's only one and already writable. no borrow it's a concrete type)
	{
		input: {},
		output: { external { $field_name:ident : $ty:ty = $value:expr; } },
		args: $args:tt
	} => {
		$value
	};

	// at least one non-capturing expression, no capturing expressions, any amount of literals (but not one non-capturing, writable expression because it's already covered)
	{
		input: {},
		output: { $(internal $internal:tt,)+ },
		args: {
			lifetime: $lifetime:lifetime,
			optional_lifetime: $($optional_lifetime:lifetime)?,
			reference: { $($reference:tt)? },
		}
	} => {$($reference)? {
		struct W;

		impl $crate::writable::Writable for W {
			$crate::impl_for_writable_internal! { $(internal $internal,)+ }
		}

		W
	}};

	// combination of sources (excluding ones that are already covered above) and where the capturing values all have concrete types
	{
		input: {},
		output: { $(internal $internal0:tt,)* $(external { $field_name:ident, generic: , ty: $ty:ty, = $value:expr; $fmt_args:tt }, $(internal $internal:tt,)*)+ },
		args: {
			lifetime: $lifetime:lifetime,
			optional_lifetime: $($optional_lifetime:lifetime)?,
			reference: { $($reference:tt)? },
		}
	} => {$($reference)? {
		// import all the traits with the name of the field as the name.
		// it's in a module because otherwise the names clash with the generics and also so that they are private
		mod traits_ {
			$(
				$crate::use_internal_writable_trait_from_fmt_args!(pub use $fmt_args as $field_name;);
			)+
		}

		#[allow(non_camel_case_types)]
		struct W<$($optional_lifetime)?> {
			$($field_name : $ty ),+
		}

		#[allow(non_camel_case_types)]
		impl<$($optional_lifetime)?> $crate::writable::Writable for W<$($optional_lifetime)?> {
			$crate::impl_for_writable_internal! {
				$(internal $internal0,)*
				$(external { $field_name; $fmt_args }, $(internal $internal,)*)+
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
			)+
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

		// import all the traits with the name of the field as the name.
		// it's in a module because otherwise the names clash with the generics and also so that they are private
		mod traits_ {
			$(
				$crate::use_internal_writable_trait_from_fmt_args!(pub use $fmt_args as $field_name;);
			)+
		}

		#[allow(non_camel_case_types)]
		struct W<$lifetime, $($($generic : traits_::$field_name + ?Sized, )?)+> {
			$($field_name : $(&$lifetime $generic)? $($ty)? ),+
		}

		#[allow(non_camel_case_types)]
		impl<$lifetime, $($($generic : traits_::$field_name + ?Sized, )?)+> $crate::writable::Writable for W<$lifetime, $($($generic, )?)+> {
			$crate::impl_for_writable_internal! {
				$(internal $internal0,)*
				$(external { $field_name; $fmt_args }, $(internal $internal,)*)+
			}
		}

		W {
			$($field_name :
				$({
					$crate::noop!($generic);
					use traits_::$field_name as _;
					($value).borrow_writable_internal()
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
			"see documentation for the `fmt` macro\n",
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
	} => {'block_label: {
		use $crate::write::WriteInternal as _;
		let writer = $writer.borrow_write_internal();
		$(
			$crate::write_fmt_single_break_internal!('block_label writer => $fmt);
		)+
		::core::result::Result::Ok(())
	}};
	{
		input: {},
		output: { $($fmt:tt,)* },
		args: {
			writer: $writer:expr,
			ignore_err: true,
		}
	} => {'block_label: {
		use $crate::write::WriteInternal as _;
		let writer = $writer.borrow_write_internal();
		$(
			$crate::write_fmt_single_break_ignore_err_internal!('block_label writer => $fmt);
		)+
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
	{ { ? # } => $($tt:tt)* } => {
		$crate::write_fmt_internal! {
			input: { $($tt, )* },
			output: {},
			args: {
				writer: (::std::io::stdout()),
				ignore_err: false,
			}
		}
	};
	{ { # } => $($tt:tt)* } => {
		$crate::write_fmt_internal! {
			input: { $($tt, )* },
			output: {},
			args: {
				writer: (::std::io::stdout()),
				ignore_err: true,
			}
		}
	};
	{ { ? $writer:expr } => $($tt:tt)* } => {
		$crate::write_fmt_internal! {
			input: { $($tt, )* },
			output: {},
			args: {
				writer: $writer,
				ignore_err: false,
			}
		}
	};
	{ { $writer:expr } => $($tt:tt)* } => {
		$crate::write_fmt_internal! {
			input: { $($tt, )* },
			output: {},
			args: {
				writer: $writer,
				ignore_err: true,
			}
		}
	};
	{ [impl_for_writable] => $($tt:tt)* } => {
		$crate::fmt_internal! {
			input: { $($tt, )* },
			output: {},
			args: {
				writable_fn_name: write_to,
			}
		}
	};
	{ [impl_for_writable_debug] => $($tt:tt)* } => {
		$crate::fmt_internal! {
			input: { $($tt, )* },
			output: {},
			args: {
				writable_fn_name: write_to_debug,
			}
		}
	};
	{ [impl_for_writable with: $fn_name:ident] => $($tt:tt)* } => {
		$crate::fmt_internal! {
			input: { $($tt, )* },
			output: {},
			args: {
				writable_fn_name: $fn_name,
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
macro_rules! fmt_no_ref {
	($($tt:tt)*) => {
		$crate::fmt_advanced!({} => $($tt)*)
	};
}

#[macro_export]
macro_rules! write_fmt {
	(# => $($tt:tt)*) => {
		$crate::fmt_advanced!({ # } => $($tt)*)
	};
	(? # => $($tt:tt)*) => {
		$crate::fmt_advanced!({ ? # } => $($tt)*)
	};
	($writer:expr => $($tt:tt)*) => {
		$crate::fmt_advanced!({ $writer } => $($tt)*)
	};
	(? $writer:expr => $($tt:tt)*) => {
		$crate::fmt_advanced!({ ? $writer } => $($tt)*)
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

// TODO: return closure
#[macro_export]
macro_rules! fmt_fn {
    () => {};
}

#[macro_export]
macro_rules! default_token {
    ($value:expr, $default:expr) => {
        $value
    };

    (, $default:expr) => {
        $default
    };

    ($value:expr, $default:ty) => {
        $value
    };

    (, $default:ty) => {
        $default
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

    use crate::writable::{ToString, Writable, WritableDebug};

    struct Struct {
        a: i32,
        b: bool,
    }

    impl WritableDebug for Struct {
        fmt_struct_debug!([impl_for_writable_debug] => Struct; { a, b, });
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

    let a = &mut *String::from("abc");
    let s = fmt!("123" [xyz!()] "abc" {a} "abc");
    let s0 = ToString::to_string(s);
    assert_eq!(s0.len(), s.len_hint());

    let a = String::from("abc");
    let s = fmt!("123" [xyz!()] "abc" {a} "abc");
    let s0 = ToString::to_string(s);
    assert_eq!(s0.len(), s.len_hint());

    let a = &String::from("abc");
    let s = fmt!("123" [xyz!()] "abc" {a} "abc");
    let s0 = ToString::to_string(s);
    assert_eq!(s0.len(), s.len_hint());

    let a = &mut String::from("abc");
    let s = fmt!("123" [xyz!()] "abc" {a} "abc");
    let s0 = ToString::to_string(s);
    assert_eq!(s0.len(), s.len_hint());

    let a = &mut String::from("abc");
    let s = fmt_no_ref!("123" [xyz!()] "abc" {a} "abc");
    let s0 = ToString::to_string(&s);
    assert_eq!(s0.len(), s.len_hint());

    let a: Box<str> = Box::from("abc");
    let s = fmt!("123" [xyz!()] "abc" {a} "abc");
    let s0 = ToString::to_string(s);
    assert_eq!(s0.len(), s.len_hint());

    let a = &3_i32;
    let s = fmt!("123" [xyz!()] "abc" {a} "abc");
    let s0 = ToString::to_string(s);

    let a = 3_i32;
    let w = fmt!("123" [xyz!()] "abc" {a} "abc");
    let s0 = ToString::to_string(w);

    let a = 3_i32;
    let w = fmt!("123" [xyz!()] "abc" {a: i32} "abc");
    let s0 = ToString::to_string(w);

    const _S: &str = fmt!("123" [xyz!()] "abc" "abc" 123);

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

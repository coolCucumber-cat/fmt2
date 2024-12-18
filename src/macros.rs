#[macro_export]
macro_rules! noop {
    ($($tt:tt)*) => {};
}

#[macro_export]
macro_rules! use_internal_writable_trait_from_fmt_args {
    (use { trait $tr:path } as $name:ident;) => {
        use $tr as $name;
    };
    (use {} as $name:ident;) => {
        use $crate::writable::WritableInternal as $name;
    };
    (use { ? } as $name:ident;) => {
        use $crate::writable::WritableDebugInternal as $name;
    };
    (use { b } as $name:ident;) => {
        use $crate::writable::WritableBinaryInternal as $name;
    };
    (use { h } as $name:ident;) => {
        use $crate::writable::WritableHexadecimalInternal as $name;
    };
}

#[macro_export]
macro_rules! write_fmt_single_internal {
	($writer:expr => { $value:expr; $($fmt_args:tt)* }) => {{
		$crate::use_internal_writable_trait_from_fmt_args!(use { $($fmt_args)* } as A;);
		($value).write_to_internal($writer)?;
	}};

	($writer:expr => [$("", )*]) => {{
		compile_error!("unreachable. dev error or bug using macro");
	}};

	($writer:expr => [$($value:expr, )+]) => {{
		const S: &str = ::core::concat!($($value),+);
		if S.len() != 0 {
			$crate::write::Write::write_str($writer, S)?;
		}
	}};
}

#[macro_export]
macro_rules! len_hint_fmt_single_internal {
	({ $value:expr; $($fmt_args:tt)* }) => {{
		$crate::use_internal_writable_trait_from_fmt_args!(use { $($fmt_args)* } as A;);
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
		input: { $([$($prev:expr),* $(,)?], )* { $field_name:ident = $literal:literal $(;)? }, $($inputs:tt, )* },
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
		input: { { $field_name:ident = $value:expr; trait $tr:path }, $($inputs:tt, )* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal! {
			input: { $($inputs, )* },
			output: { $($outputs)*
				external { $field_name, generic: $field_name, ty: , = $value; trait $tr },
			},
			args: $args
		}
	};
	// capturing expression with concrete type
	{
		input: { { $field_name:ident : $ty:ty = $value:expr; trait $tr:path }, $($inputs:tt, )* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal! {
			input: { $($inputs, )* },
			output: { $($outputs)*
				external { $field_name, generic: , ty: $ty, = $value; trait $tr },
			},
			args: $args
		}
	};
	// ... capturing expression and parsing fmt_args to get trait
	{
		input: { { $field_name:ident $(: $ty:ty)? = $value:expr $(;)? }, $($inputs:tt, )* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal! {
			input: { { $field_name $(: $ty)? = $value; trait $crate::writable::WritableInternal }, $($inputs, )* },
			output: { $($outputs)* },
			args: $args
		}
	};
	{
		input: { { $field_name:ident $(: $ty:ty)? = $value:expr; ? }, $($inputs:tt, )* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal! {
			input: { { $field_name $(: $ty)? = $value; trait $crate::writable::WritableDebugInternal }, $($inputs, )* },
			output: { $($outputs)* },
			args: $args
		}
	};
	{
		input: { { $field_name:ident $(: $ty:ty)? = $value:expr; b }, $($inputs:tt, )* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal! {
			input: { { $field_name $(: $ty)? = $value; trait $crate::writable::WritableBinaryInternal }, $($inputs, )* },
			output: { $($outputs)* },
			args: $args
		}
	};
	{
		input: { { $field_name:ident $(: $ty:ty)? = $value:expr; h }, $($inputs:tt, )* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal! {
			input: { { $field_name $(: $ty)? = $value; trait $crate::writable::WritableHexadecimalInternal }, $($inputs, )* },
			output: { $($outputs)* },
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

	// only one non-capturing, writable expression (no wrapper struct, since it's only one and already writable)
	{
		input: {},
		output: { internal { $value:expr; } },
		args: {
			lifetime: $lifetime:lifetime,
			optional_lifetime: $($optional_lifetime:lifetime)?,
			reference: { & },
		}
	} => {{
		use $crate::writable::WritableInternal as A;
		($value).borrow_writable_internal()
	}};
	{
		input: {},
		output: { internal { $value:expr; } },
		args: $args:tt
	} => {{
		$value
	}};
	// only one capturing, writable expression (no wrapper struct, since it's only one and already writable)
	{
		input: {},
		output: { external { $field_name:ident = $value:expr; } },
		args: {
			lifetime: $lifetime:lifetime,
			optional_lifetime: $($optional_lifetime:lifetime)?,
			reference: { & },
		}
	} => {{
		use $crate::writable::WritableInternal as A;
		($value).borrow_writable_internal()
	}};
	{
		input: {},
		output: { external { $field_name:ident = $value:expr; } },
		args: $args:tt
	} => {{
		$value
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
			#[inline]
			fn write_to<W>(&self, w: &mut W) -> Result<(), W::Error>
				where
					W: $crate::write::Write + ?Sized {
				$(
					$crate::write_fmt_single_internal!(w => $internal);
				)+

				::core::result::Result::Ok(())
			}

			#[inline]
			fn len_hint(&self) -> usize {
				0
				$(
					+ $crate::len_hint_fmt_single_internal!($internal)
				)+
			}
		}

		W
	}};

	// combination of sources (excluding ones that are already covered above) and where the capturing values all have concrete types
	{
		input: {},
		output: { $(internal $internal0:tt,)* $(external { $field_name:ident, generic: , ty: $ty:ty, = $value:expr; trait $tr:path }, $(internal $internal:tt,)*)+ },
		args: {
			lifetime: $lifetime:lifetime,
			optional_lifetime: $($optional_lifetime:lifetime)?,
			reference: { $($reference:tt)? },
		}
	} => {$($reference)? {
		#[allow(non_camel_case_types)]
		struct W<$($optional_lifetime)?> {
			$($field_name : $ty ),+
		}

		#[allow(non_camel_case_types)]
		impl<$($optional_lifetime)?> $crate::writable::Writable for W<$($optional_lifetime)?> {
			#[inline]
			fn write_to<W>(&self, w: &mut W) -> Result<(), W::Error>
				where
					W: $crate::write::Write + ?Sized {
				$(
					$crate::write_fmt_single_internal!(w => $internal0);
				)*
				$(
					$crate::write_fmt_single_internal!(w => { self.$field_name; trait $tr });
					$(
						$crate::write_fmt_single_internal!(w => $internal);
					)*
				)+

				::core::result::Result::Ok(())
			}

			#[inline]
			fn len_hint(&self) -> usize {
				0
				$(
					+ $crate::len_hint_fmt_single_internal!($internal0)
				)*
				$(
					+ $crate::len_hint_fmt_single_internal!({ self.$field_name; trait $tr })
					$(
						+ $crate::len_hint_fmt_single_internal!($internal)
					)*
				)+
			}
		}

		W {
			$($field_name : $value),*
		}
	}};
	// combination of sources (excluding ones that are already covered above)
	{
		input: {},
		output: { $(internal $internal0:tt,)* $(external { $field_name:ident, generic: $($generic:ident)?, ty: $($ty:ty)?, = $value:expr; trait $tr:path }, $(internal $internal:tt,)*)+ },
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
		struct W<$lifetime, $($($generic : $tr + ?Sized, )?)+> {
			$($field_name : $(&$lifetime $generic)? $($ty)? ),+
		}

		#[allow(non_camel_case_types)]
		impl<$lifetime, $($($generic : $tr + ?Sized, )?)+> $crate::writable::Writable for W<$lifetime, $($($generic, )?)+> {
			#[inline]
			fn write_to<W>(&self, w: &mut W) -> Result<(), W::Error>
				where
					W: $crate::write::Write + ?Sized {
				$(
					$crate::write_fmt_single_internal!(w => $internal0);
				)*
				$(
					$crate::write_fmt_single_internal!(w => { self.$field_name; trait $tr });
					$(
						$crate::write_fmt_single_internal!(w => $internal);
					)*
				)+

				::core::result::Result::Ok(())
			}

			#[inline]
			fn len_hint(&self) -> usize {
				0
				$(
					+ $crate::len_hint_fmt_single_internal!($internal0)
				)*
				$(
					+ $crate::len_hint_fmt_single_internal!({ self.$field_name; trait $tr })
					$(
						+ $crate::len_hint_fmt_single_internal!($internal)
					)*
				)+
			}
		}

		W {
			$($field_name :
				$({
					$crate::noop!($generic);
					use $tr as A;
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
macro_rules! fmt {
	($($tt:tt)+) => {
		$crate::fmt_internal! {
			input: { $($tt, )+ },
			output: {},
			args: {
				lifetime: 'a,
				optional_lifetime:,
				reference: { & },
			}
		}
	};
}

#[macro_export]
macro_rules! fmt_no_ref {
	($($tt:tt)+) => {
		$crate::fmt_internal! {
			input: { $($tt, )+ },
			output: {},
			args: {
				lifetime: 'a,
				optional_lifetime:,
				reference: {},
			}
		}
	};
}

#[doc(hidden)]
#[macro_export]
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
			output: { $($outputs)* internal [$($literal, )*], },
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

	// nothing
	{
		input: {},
		output: {},
		args: $args:tt
	} => {{
		::core::result::Ok(())
	}};
	// error
	{
		input: {},
		output: { $([$("", )*], )* },
		args: $args:tt
	} => {{
		compile_error!("unreachable. dev error in macro");
	}};
	// only literals (concat)
	{
		input: {},
		output: { [$($literals:expr, )+], },
		args: {
			writer: $writer:expr,
			ignore_err: $ignore_err:tt,
		}
	} => {try {
		$crate::write_fmt_single_internal!($writer => [$($literals:expr, )+])
	}};
	// error
	{
		input: {},
		output: { $([$($literals:expr, )*], )* },
		args: $args:tt
	} => {{
		compile_error!("unreachable. dev error in macro");
	}};

	// terminate recursion
	{
		input: {},
		output: { $($fmt:tt,)+ },
		args: {
			writer: $writer:expr,
			ignore_err: $ignore_err:tt,
		}
	} => {try {
		$(
			$crate::write_fmt_single_internal!($writer => $fmt)
		)+
	}};
}

#[macro_export]
macro_rules! write_fmt {
	($writer:expr => $($tt:tt)*) => {
		$crate::write_fmt_internal! {
			input: { $($tt, )* },
			output: {},
			args: {}
		}
	};
}

#[macro_export]
macro_rules! fmt_struct {
	($value:expr => $name:ident { $field0:ident $([$($tt0:tt)*])? $(, $field:ident $([$($tt:tt)*])?)* $(,)? }) => {
		$crate::fmt!([stringify!($name)] " { " [stringify!($field0)] ": " {$field0 = $value.$field0;$($($tt0)*)?} $(", " [stringify!($field)] ": " {$field = $value.$field;$($($tt)*)?})* " }")
	};

	($name:ident; { $field0:ident : $tt0:tt $(, $field:ident : $tt:tt)* $(,)? }) => {
		$crate::fmt!([stringify!($name)] " { " [stringify!($field0)] ": " $tt0 $(", " [stringify!($field)] ": " $tt)* " }")
	};

	($value:expr => $name:ident {}) => {
		$crate::fmt!([stringify!($name)] " {}")
	};
}

#[macro_export]
macro_rules! fmt_tuple_struct {
	($value:expr => $name:ident ($field0:ident $tfield0:tt $([$($tt0:tt)*])? $(, $field:ident $tfield:tt $([$($tt:tt)*])?)* $(,)?)) => {
		$crate::fmt!([stringify!($name)] "(" {$field0 = $value.$tfield0;$($($tt0)*)?} $(", " {$field = $value.$tfield;$($($tt)*)?})* ")")
	};

	($name:ident; ($tt0:tt $(, $tt:tt)* $(,)?) ) => {
		$crate::fmt!([stringify!($name)] "(" $tt0 $(", " $tt)* ")")
	};

	($value:expr => $name:ident () ) => {
		$crate::fmt!([stringify!($name)] "()")
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

    use crate::writable::{ToString, Writable};

    struct Struct {
        a: i32,
        b: bool,
    }

    struct Tuple(i32, bool);

    let struct_ = Struct { a: 12, b: true };
    let s = fmt_struct!(struct_ => Struct { a [], b });
    let s0 = s.to_string();
    assert_eq!(s0, "Struct { a: 12, b: true }");

    let tuple = Tuple(99, true);
    let s = fmt_tuple_struct!(tuple => Tuple (a 0, b 1));
    let s0 = s.to_string();
    assert_eq!(s0, "Tuple(99, true)");

    const S: &str = fmt_struct!(Struct; { a: {a = 234}, b: {b = false} });
    let s0 = ToString::to_string(S);
    assert_eq!(s0, "Struct { a: 234, b: false }");

    const S1: &str = fmt_tuple_struct!(Tuple; ({a = 234}, {b = false}));
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

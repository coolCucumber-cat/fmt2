macro_rules! fmt_internal_concat_literals {
	($(""),* $(,)?) => {};

	($($l:expr),+ $(,)?) => {
		concat!($($l),+)
	};
}
pub(crate) use fmt_internal_concat_literals;

macro_rules! fmt_internal_write_literals {
	($writer:expr => $(""),* $(,)?) => {};

	($writer:expr => $($l:expr),+ $(,)?) => {{
		const S: &str = concat!($($l),+);
		const LEN: usize = S.len();
		const WRITE: bool = LEN != 0;

		if WRITE {
			$crate::write::Write::write_str($writer, S)?;
		};
	}};
}
pub(crate) use fmt_internal_write_literals;

macro_rules! fmt_internal_write_value_2 {
	($writer:expr, $value:expr => {}) => {{
		$crate::write::Write::write($writer, $value)?
	}};
	($writer:expr, $value:expr => {?}) => {{
		$crate::write::Write::write_debug($writer, $value)?
	}};
	($writer:expr, $value:expr => {display}) => {{
		$crate::write::Write::write_fmtdisplay($writer, $value)?
	}};
	($writer:expr, $value:expr => {debug}) => {{
		$crate::write::Write::write_fmtdebug($writer, $value)?
	}};
	($writer:expr, $value:expr => {$($tt:tt)*}) => {{
		compile_error!(concat!("invalid formatting arguments: ", $(stringify!($tt)),*))
	}};
}
pub(crate) use fmt_internal_write_value_2;

macro_rules! fmt_internal_write_value {
	($writer:expr, $struct_:expr => { $field_name:ident $(; $($tt:tt)*)? }) => {{
		$crate::macros::fmt_internal_write_value_2!($writer, $struct_.$field_name => { $($($tt)*)? })
	}};

	($writer:expr $(, $struct_:expr)? => ($expr:expr $(; $($tt:tt)*)?)) => {{
		use ::core::ops::Deref;
		$crate::macros::fmt_internal_write_value_2!($writer, (&$expr).deref() => { $($($tt)*)? })
	}};
}
pub(crate) use fmt_internal_write_value;

macro_rules! fmt_internal {
	// recursion

	// literals
	{
		input: { $input:literal, $($inputs:tt, )* },
		// input: { $($input:literal, )+ $($inputs:tt, )* },
		output: { $($outputs:tt)* },
		output_fields: { $($output_fields:tt)* }
	} => {
		$crate::macros::fmt_internal! {
			input: { $($inputs, )* },
			output: { $($outputs)* ,[$input] },
			output_fields: { $($output_fields)* }
		}
	};
	{
		input: { ($input:literal $(;)?), $($inputs:tt, )* },
		output: { $($outputs:tt)* },
		output_fields: { $($output_fields:tt)* }
	} => {
		$crate::macros::fmt_internal! {
			input: { $($inputs, )* },
			output: { $($outputs)* ,[$input] },
			output_fields: { $($output_fields)* }
		}
	};
	{
		input: { { $input:literal $(; $input_field_name:ident)? }, $($inputs:tt, )* },
		output: { $($outputs:tt)* },
		output_fields: { $($output_fields:tt)* }
	} => {
		$crate::macros::fmt_internal! {
			input: { $($inputs, )* },
			output: { $($outputs)* ,[$input] },
			output_fields: { $($output_fields)* }
		}
	};

	// expressions that are literals after macro expansion (can be concatenated with `concat!()`)
	{
		input: { [$($input:expr)*], $($inputs:tt, )* },
		output: { $($outputs:tt)* },
		output_fields: { $($output_fields:tt)* }
	} => {
		$crate::macros::fmt_internal! {
			input: { $($inputs, )* },
			output: { $($outputs)* ,[$($input)*] },
			output_fields: { $($output_fields)* }
		}
	};

	// expressions that don't capture any external variables (consts, statics, fn call)
	{
		input: { ($input:expr $(; $($input_tt:tt)*)?), $($inputs:tt, )* },
		output: { $($outputs:tt)* },
		output_fields: { $($output_fields:tt)* }
	} => {
		$crate::macros::fmt_internal! {
			input: { $($inputs, )* },
			output: { $($outputs)* ;($input; $($($input_tt:tt)*)?) },
			output_fields: { $($output_fields)* }
		}
	};

	// expressions
	{
		input: { { $input_value:expr ; $input_field_name:ident $($input_tt:tt)* }, $($inputs:tt, )* },
		output: { $($outputs:tt)* },
		output_fields: { $($output_fields:tt)* }
	} => {
		$crate::macros::fmt_internal! {
			input: { $($inputs, )* },
			output: { $($outputs)* ;{ $input_field_name $($input_tt)* } },
			output_fields: { $($output_fields)* { $input_field_name : $input_value } }
		}
	};
	// same as above but automatic name using variable as name and value
	{
		input: { { $input_field_name:ident $($input_tt:tt)* }, $($inputs:tt, )* },
		output: { $($outputs:tt)* },
		output_fields: { $($output_fields:tt)* }
	} => {
		$crate::macros::fmt_internal! {
			input: { $($inputs, )* },
			output: { $($outputs)* ;{ $input_field_name $($input_tt)* } },
			output_fields: { $($output_fields)* { $input_field_name : $input_field_name } }
		}
	};

	// error (macros must be in square brackets)
	{
		input: { $name:ident!$args:tt, $($inputs:tt, )* },
		output: { $($outputs:tt)* },
		output_fields: { $($output_fields:tt)* }
	} => {{
		compile_error!(concat!(
			"macros must be in [square brackets]\n",
			stringify!($name), "!", stringify!($args),
		));
	}};
	// error
	{
		input: { $args:tt, $($inputs:tt, )* },
		output: { $($outputs:tt)* },
		output_fields: { $($output_fields:tt)* }
	} => {{
		compile_error!(concat!(
			"expressions must be either valid literals or in (round), {curly} or [square] brackets\n",
			"see documentation for the `fmt` macro\n",
			stringify!($args),
		));
	}};

	// terminate recursion

	// only literals
	{
		input: {},
		output: { $(,[$($output_literals:expr)*])* },
		output_fields: {}
	} => {
		$crate::macros::fmt_internal_concat_literals!($($($output_literals, )*)*)
	};

	// only one expression that don't capture any external variables
	{
		input: {},
		output: { $(,[$("")*])* ;($output:expr $(;)?) $(,[$("")*])* },
		output_fields: {}
	} => {{
		use ::core::ops::Deref;
		(&*$output).deref()
	}};

	// only one expression
	{
		input: {},
		output: { $([,$("")*])* { $output:ident $(;)? }; $(,[$("")*])* },
		output_fields: { { $output_field_name:ident : $output_field_value:expr } }
	} => {{
		use ::core::ops::Deref;
		(&*$output_field_value).deref()
	}};

	// combination of writable sources, no captures
		{
		input: {},
		output: { $(,[$($output_literals_start:expr)*])* $(;$output_values:tt $(,[$($output_literals:expr)*])*)+ },
		output_fields: {}
	} => {{
		struct W;

		impl $crate::writable::Writable for W {
			fn write_to<W>(&self, w: &mut W) -> Result<(), W::Error>
				where
					W: $crate::write::Write + ?Sized {
				$crate::macros::fmt_internal_write_literals!(w => $($($output_literals_start,)*)*);
				$(
					$crate::macros::fmt_internal_write_value!(w => $output_values);

					$crate::macros::fmt_internal_write_literals!(w => $($($output_literals,)*)*);
				)+

				::core::result::Result::Ok(())
			}
		}

		W
	}};


	// combination of writable sources
	{
		input: {},
		output: { $(,[$($output_literals_start:expr)*])* $(;$output_values:tt $(,[$($output_literals:expr)*])*)+ },
		output_fields: { $({ $output_field_names:ident : $output_field_values:expr })+ }
	} => {{
		use ::core::ops::Deref;

		// for syntax highlighting
		{
			$(let $output_field_names: u8;)*
		}

		#[allow(non_camel_case_types)]
		struct W<'a, $($output_field_names : $crate::writable::Writable + ?Sized),+> {
			$($output_field_names : &'a $output_field_names),+
		}

		#[allow(non_camel_case_types)]
		impl<'a, $($output_field_names : $crate::writable::Writable + ?Sized),+> $crate::writable::Writable for W<'a, $($output_field_names),+> {
			fn write_to<W>(&self, w: &mut W) -> Result<(), W::Error>
				where
					W: $crate::write::Write + ?Sized {
				$crate::macros::fmt_internal_write_literals!(w => $($($output_literals_start,)*)*);
				$(
					$crate::macros::fmt_internal_write_value!(w, self => $output_values);

					$crate::macros::fmt_internal_write_literals!(w => $($($output_literals,)*)*);
				)+

				::core::result::Result::Ok(())
			}
		}
		// $(let $output_field_names = (&*$output_field_values).deref();)*
		// W {
		// 	$($output_field_names : $output_field_names),*
		// }
		use $crate::utils::DerefIgnoreMutForMut;
		use $crate::utils::DerefIgnoreMut;
		W {
			$($output_field_names : $output_field_values.deref_ignore_mut()),*
			// $($output_field_names : $output_field_values.deref()),*
			// $($output_field_names : Deref::deref(&$output_field_values)),*
			// $($output_field_names : (&*$output_field_values)),*
		}
	}};
}
pub(crate) use fmt_internal;

#[macro_export]
macro_rules! fmt {
	($($tt:tt)+) => {
		$crate::macros::fmt_internal! {
			input: { $($tt, )+ },
			output: {},
			output_fields: {}
		}
	};
}

// TODO: return closure
macro_rules! fmt_fn {
    () => {};
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
    use crate::writable::{ToString, Writable};

    macro_rules! xyz {
        () => {
            "XYZ"
        };
    }

    let a = "abc";
    let s = fmt!("123" [xyz!()] "abc" {a} "abc");
    s.to_string();

    // let a = &mut "abc";
    // let s = fmt!("123" [xyz!()] "abc" {a} "abc");
    // s.to_string();

    let a = String::from("abc");
    let s = fmt!("123" [xyz!()] "abc" {a} "abc");
    s.to_string();

    let a = &String::from("abc");
    let s = fmt!("123" [xyz!()] "abc" {a} "abc");
    s.to_string();

    let a = &mut String::from("abc");
    let s = fmt!("123" [xyz!()] "abc" {a} "abc");
    s.to_string();

    let a: Box<str> = Box::from("abc");
    let s = fmt!("123" [xyz!()] "abc" {a} "abc");
    s.to_string();

    let a = &3;
    let s = fmt!("123" [xyz!()] "abc" {a} "abc");
    s.to_string();

    const S: &str = fmt!("123" [xyz!()] "abc" "abc" 123);

    const A: i32 = 32;
    let s = fmt!("123" [xyz!()] "abc" {a} "123" (&A) "abc");
    fn const_fn(a: i32, b: i32) -> i32 {
        a + b
    }
    let s = fmt!("123" [xyz!()] "abc" (A) "abc" {456});
    let s = fmt!("123" [xyz!()] "abc" (const_fn(1, 2)) "abc");

    fn post_deref<T>(t: &T) -> &T
    where
        T: Writable,
    {
        t
    }
}

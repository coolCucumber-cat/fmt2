#[macro_export]
macro_rules! fmt_internal_concat_literals {
	($(""),* $(,)?) => {};

	($($l:expr),+ $(,)?) => {
		concat!($($l),+)
	};
}

#[macro_export]
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

#[macro_export]
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
	($writer:expr, $value:expr => {$($tt:tt)*}) => {
		compile_error!(concat!("invalid formatting arguments: ", $(stringify!($tt)),*))
	};
}

#[macro_export]
macro_rules! fmt_internal_write_value {
	($writer:expr, $struct_:expr => { $field_name:ident; $($tt:tt)* }) => {{
		$crate::fmt_internal_write_value_2!($writer, $struct_.$field_name => { $($($tt)*)? })
	}};

	($writer:expr $(, $struct_:expr)? => ($expr:expr; $($tt:tt)*)) => {{
		use $crate::utils::DerefForWritable;
		use $crate::utils::DerefForWritableMut;
		use $crate::utils::DerefForWritableFmt;
		// use ::core::ops::Deref;
		$crate::fmt_internal_write_value_2!($writer, ($expr).deref_for_writable() => { $($($tt)*)? })
	}};
}

#[macro_export]
macro_rules! fmt_internal {
	// recursion

	// literals
	{
		input: { $input:literal, $($inputs:tt, )* },
		// input: { $($input:literal, )+ $($inputs:tt, )* },
		output: { $($outputs:tt)* },
		output_fields: { $($output_fields:tt)* }
	} => {
		$crate::fmt_internal! {
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
		$crate::fmt_internal! {
			input: { $($inputs, )* },
			output: { $($outputs)* ,[$input] },
			output_fields: { $($output_fields)* }
		}
	};
	{
		input: { { $input_field_name:ident : $input:literal $(;)? }, $($inputs:tt, )* },
		output: { $($outputs:tt)* },
		output_fields: { $($output_fields:tt)* }
	} => {
		$crate::fmt_internal! {
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
		$crate::fmt_internal! {
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
		$crate::fmt_internal! {
			input: { $($inputs, )* },
			output: { $($outputs)* ;($input; $($($input_tt:tt)*)?) },
			output_fields: { $($output_fields)* }
		}
	};

	// expressions
	{
		input: { { $input_field_name:ident : $input_value:expr $(; $($input_tt:tt)*)? }, $($inputs:tt, )* },
		output: { $($outputs:tt)* },
		output_fields: { $($output_fields:tt)* }
	} => {
		$crate::fmt_internal! {
			input: { $($inputs, )* },
			output: { $($outputs)* ;{ $input_field_name; $($($input_tt)*)? } },
			output_fields: { $($output_fields)* { $input_field_name : $input_value } }
		}
	};
	// same as above but automatic name using variable as name and value
	{
		input: { { $input_field_name:ident $(; $($input_tt:tt)*)? }, $($inputs:tt, )* },
		output: { $($outputs:tt)* },
		output_fields: { $($output_fields:tt)* }
	} => {
		$crate::fmt_internal! {
			input: { $($inputs, )* },
			output: { $($outputs)* ;{ $input_field_name; $($($input_tt)*)? } },
			output_fields: { $($output_fields)* { $input_field_name : $input_field_name } }
		}
	};
	// same as above but automatic name using variable as name and value and also a reference
	{
		input: { { &$input_field_name:ident $(; $($input_tt:tt)*)? }, $($inputs:tt, )* },
		output: { $($outputs:tt)* },
		output_fields: { $($output_fields:tt)* }
	} => {
		$crate::fmt_internal! {
			input: { $($inputs, )* },
			output: { $($outputs)* ;{ $input_field_name; $($($input_tt)*)? } },
			output_fields: { $($output_fields)* { $input_field_name : &$input_field_name } }
		}
	};

	// error (macros must be in square brackets)
	{
		input: { $name:ident!$args:tt, $($inputs:tt, )* },
		output: { $($outputs:tt)* },
		output_fields: { $($output_fields:tt)* }
	} => {
		compile_error!(concat!(
			"macros must be in [square brackets]\n",
			stringify!($name), "!", stringify!($args),
		));
	};
	// error
	{
		input: { $args:tt, $($inputs:tt, )* },
		output: { $($outputs:tt)* },
		output_fields: { $($output_fields:tt)* }
	} => {
		compile_error!(concat!(
			"expressions must be either valid literals or in (round), {curly} or [square] brackets\n",
			"see documentation for the `fmt` macro\n",
			stringify!($args),
		));
	};

	// terminate recursion

	// only literals
	{
		input: {},
		output: { $(,[$($output_literals:expr)*])* },
		output_fields: {}
	} => {
		$crate::fmt_internal_concat_literals!($($($output_literals, )*)*)
	};

	// only one expression that don't capture any external variables
	{
		input: {},
		output: { $(,[$("")*])* ;($output:expr $(;)?) $(,[$("")*])* },
		output_fields: {}
	} => {{
		use $crate::utils::DerefForWritableMut;
		use $crate::utils::DerefForWritableFmt;
		use $crate::utils::DerefForWritable;
		($output).deref_for_writable()
	}};

	// only one expression
	{
		input: {},
		output: { $([,$("")*])* { $output:ident $(;)? }; $(,[$("")*])* },
		output_fields: { { $output_field_name:ident : $output_field_value:expr } }
	} => {{
		use $crate::utils::DerefForWritableMut;
		use $crate::utils::DerefForWritableFmt;
		use $crate::utils::DerefForWritable;
		($output_field_value).deref_for_writable()
	}};

	// combination of writable sources, no captures
		{
		input: {},
		output: { $(,[$($output_literals_start:expr)*])* $(;$output_values:tt $(,[$($output_literals:expr)*])*)+ },
		output_fields: {}
	} => {{
		struct W;

		impl $crate::writable::Writable for W {
			#[inline]
			fn write_to<W>(&self, w: &mut W) -> Result<(), W::Error>
				where
					W: $crate::write::Write + ?Sized {
				$crate::fmt_internal_write_literals!(w => $($($output_literals_start,)*)*);
				$(
					$crate::fmt_internal_write_value!(w => $output_values);

					$crate::fmt_internal_write_literals!(w => $($($output_literals,)*)*);
				)+

				::core::result::Result::Ok(())
			}
		}

		impl $crate::utils::DerefForWritableFmt for W {
			type Target = Self;

			#[inline]
			fn deref_for_writable(&self) -> &Self::Target {
				self
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
		// use ::core::ops::Deref;

		// for syntax highlighting
		#[allow(unused)]
		{
			$(let $output_field_names: u8;)*
		}

		#[allow(non_camel_case_types)]
		struct W<'a, $($output_field_names : $crate::writable::Writable + ?Sized),+> {
			$($output_field_names : &'a $output_field_names),+
		}

		#[allow(non_camel_case_types)]
		impl<'a, $($output_field_names : $crate::writable::Writable + ?Sized),+> $crate::writable::Writable for W<'a, $($output_field_names),+> {
			#[inline]
			fn write_to<W>(&self, w: &mut W) -> Result<(), W::Error>
				where
					W: $crate::write::Write + ?Sized {
				$crate::fmt_internal_write_literals!(w => $($($output_literals_start,)*)*);
				$(
					$crate::fmt_internal_write_value!(w, self => $output_values);

					$crate::fmt_internal_write_literals!(w => $($($output_literals,)*)*);
				)+

				::core::result::Result::Ok(())
			}
		}

		#[allow(non_camel_case_types)]
		impl<'a, $($output_field_names : $crate::writable::Writable + ?Sized),+> $crate::utils::DerefForWritableFmt for W<'a, $($output_field_names),+> {
			type Target = Self;

			#[inline]
			fn deref_for_writable(&self) -> &Self::Target {
				self
			}
		}

		// 		#[allow(non_camel_case_types)]
		// 		impl<'a, $($output_field_names : $crate::writable::Writable + ?Sized),+> ::core::ops::Deref for W<'a, $($output_field_names),+> {
		//     		type Target = Self;
		//
		//			#[inline]
		//     		fn deref(&self) -> &Self::Target {
		// 				self
		//     		}
		// 		}

		use $crate::utils::DerefForWritableMut;
		use $crate::utils::DerefForWritableFmt;
		use $crate::utils::DerefForWritable;
		W {
			$($output_field_names : ($output_field_values).deref_for_writable()),*
		}
	}};
}

#[macro_export]
macro_rules! fmt {
	($($tt:tt)+) => {
		$crate::fmt_internal! {
			input: { $($tt, )+ },
			output: {},
			output_fields: {}
		}
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

    macro_rules! xyz {
        () => {
            "XYZ"
        };
    }

    let a = "abc";
    let s = fmt!("123" [xyz!()] "abc" {a} "abc");

    let a = &mut *String::from("abc");
    let s = fmt!("123" [xyz!()] "abc" {a} "abc");
    let s0 = ToString::to_string(&s);

    let a = String::from("abc");
    let s = fmt!("123" [xyz!()] "abc" {a} "abc");
    let s0 = ToString::to_string(&s);

    let a = &String::from("abc");
    let s = fmt!("123" [xyz!()] "abc" {a} "abc");
    let s0 = ToString::to_string(&s);

    let a = &mut String::from("abc");
    let s = fmt!("123" [xyz!()] "abc" {a} "abc");
    let s0 = ToString::to_string(&s);

    let a: Box<str> = Box::from("abc");
    let s = fmt!("123" [xyz!()] "abc" {a} "abc");
    let s0 = ToString::to_string(&s);

    let a = &3_i32;
    let s = fmt!("123" [xyz!()] "abc" {a} "abc");
    let s0 = ToString::to_string(&s);

    let a = 3_i32;
    let s0 = ToString::to_string(&(fmt!("123" [xyz!()] "abc" {&a} "abc")));

    const _S: &str = fmt!("123" [xyz!()] "abc" "abc" 123);

    let a: i32 = 3;
    const A: i32 = 32;
    let s = fmt!("123" [xyz!()] "abc" {&a} "123" (&A) "abc");
    fn const_fn(a: i32, b: i32) -> i32 {
        a + b
    }
    let s = fmt!("123" [xyz!()] "abc" (&A) "abc" {d:456});
    let s = fmt!("123" [xyz!()] "abc" (&const_fn(1, 2)) "abc");
}

#[macro_export]
macro_rules! fmt_internal_write {
	($writer:expr => { $value:expr; [$ty:path] }) => {{
		use $ty as A;
		A::write_to_internal($value, $writer)?;
	}};

	($writer:expr => ($value:expr; [$ty:path])) => {{
		use $ty as A;
		A::write_to_internal($value.borrow_writable_internal(), $writer)?;
	}};

	($writer:expr => [$("", )*]) => {{
		compile_error!("unreachable. impossible to be empty");
	}};

	($writer:expr => [$($value:expr, )+]) => {{
		const S: &str = ::core::concat!($($value),+);
		if S.len() != 0 {
			$crate::write::Write::write_str($writer, S)?;
		}
	}};
}

#[macro_export]
macro_rules! fmt_internal_trait_from_args {
    () => {};
}

#[doc(hidden)]
#[macro_export]
macro_rules! fmt_internal_fmt_args {
	{
		value: $value:tt,
		fmt_args: {type $ty:path},
		input: { $($inputs:tt, )* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal! {
			input: { $($inputs, )* },
			output: { $($outputs)* expr {$value [$ty] }, },
			args: $args
		}
	};
	{
		value: $value:tt,
		fmt_args: {},
		input: { $($inputs:tt, )* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal! {
			input: { $($inputs, )* },
			output: { $($outputs)* expr {$value [$crate::writable::WritableInternal] }, },
			args: $args
		}
	};
	{
		value: $value:tt,
		fmt_args: {?},
		input: { $($inputs:tt, )* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal! {
			input: { $($inputs, )* },
			output: { $($outputs)* expr {$value [$crate::writable::WritableDebugInternal] }, },
			args: $args
		}
	};
	{
		value: $value:tt,
		fmt_args: {b},
		input: { $($inputs:tt, )* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal! {
			input: { $($inputs, )* },
			output: { $($outputs)* expr {$value [$crate::writable::WritableBinaryInternal] }, },
			args: $args
		}
	};
	{
		value: $value:tt,
		fmt_args: {h},
		input: { $($inputs:tt, )* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal! {
			input: { $($inputs, )* },
			output: { $($outputs)* expr {$value [$crate::writable::WritableHexadecimalInternal] }, },
			args: $args
		}
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! fmt_internal {
	// literals
	{
		input: { $input:literal, $($inputs:tt, )* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal! {
			input: { $($inputs, )* },
			output: { $($outputs)* literal { [$input,] }, },
			args: $args
		}
	};
	{
		input: { ($input:literal $(;)?), $($inputs:tt, )* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal! {
			input: { $($inputs, )* },
			output: { $($outputs)* literal { [$input,] }, },
			args: $args
		}
	};
	{
		input: { { $input_field_name:ident : $input:literal $(;)? }, $($inputs:tt, )* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal! {
			input: { $($inputs, )* },
			output: { $($outputs)* literal { [$input,] }, },
			args: $args
		}
	};

	// expressions that are literals after macro expansion (can be concatenated with `concat!()`)
	{
		input: { [$($input:expr)*], $($inputs:tt, )* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal! {
			input: { $($inputs, )* },
			output: { $($outputs)* literal { [$($input, )*] }, },
			args: $args
		}
	};

	// expressions that don't capture any external variables (consts, statics, fn call)
	{
		input: { ($input:expr $(; $($fmt_args:tt)*)?), $($inputs:tt, )* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal_fmt_args! {
			value: ($input),
			fmt_args: { $($($fmt_args)*)? },
			input: { $($inputs, )* },
			output: { $($outputs)* },
			args: $args
		}
	};

	// expressions
	{
		input: { { $input_field_name:ident : $input_value:expr $(; $($fmt_args:tt)*)? }, $($inputs:tt, )* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal_fmt_args! {
			value: { $input_field_name : $input_value },
			fmt_args: { $($($fmt_args)*)? },
			input: { $($inputs, )* },
			output: { $($outputs)* },
			args: $args
		}
	};

	{
		input: { { $input_field_name:ident $($input_ty:ty)? : $input_value:expr $(; $($fmt_args:tt)*)? }, $($inputs:tt, )* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal_fmt_args! {
			value: { $input_field_name $($input_ty)? : $input_value },
			fmt_args: { $($($fmt_args)*)? },
			input: { $($inputs, )* },
			output: { $($outputs)* },
			args: $args
		}
	};
	// same as above but automatic name using variable as name and value
	{
		input: { { $input_field_name:ident $($input_ty:ty)? $(; $($fmt_args:tt)*)? }, $($inputs:tt, )* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal! {
			input: { { $input_field_name $($input_ty)? : $input_field_name; $($($fmt_args)*)? }, $($inputs, )* },
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

	{
		input: {},
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal_2! {
			input: { $($outputs)+ },
			output: {},
			args: $args
		}
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! fmt_internal_2 {
	// recursion

	// literals
	{
		input: { $(literal { [$($input:expr, )*] }, )+ $(expr $inputs_a:tt, $($inputs_b:tt $inputs_c:tt,)*)? },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal_2! {
			input: { $(expr $inputs_a, $($inputs_b $inputs_c,)*)? },
			output: { $($outputs)* internal [$($($input, )*)+], },
			args: $args
		}
	};
	{
		input: { $(literal { [$("", )*] }, )+ $(expr $inputs_a:tt, $($inputs_b:tt $inputs_c:tt,)*)? },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal_2! {
			input: { $(expr $inputs_a, $($inputs_b $inputs_c,)*)? },
			output: { $($outputs)* },
			args: $args
		}
	};

	// expressions that don't capture any external variables (consts, statics, fn call)
	{
		input: { expr { ($input_value:expr) $input_fmt_args:tt }, $($inputs:tt)* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal_2! {
			input: { $($inputs)* },
			output: { $($outputs)* internal ($input_value; $input_fmt_args), },
			args: $args
		}
	};

	// expressions
	{
		input: { expr { { $input_field_name:ident : $input_value:expr } $input_fmt_args:tt }, $($inputs:tt)* },
		output: { $($outputs:tt)* },
		args: $args:tt
	} => {
		$crate::fmt_internal_2! {
			input: { $($inputs)* },
			output: { $($outputs)* external { $input_field_name : $input_value; $input_fmt_args }, },
			args: $args
		}
	};

	// terminate recursion

	// only literals
	{
		input: {},
		output: { $(internal [$("", )*], )? },
		args: $args:tt
	} => {
		""
	};
	// only literals
	{
		input: {},
		output: { internal [$($literals:expr, )*], },
		args: $args:tt
	} => {
		::core::concat!($($literals),*)
	};


	// only one expression that don't capture any external variables
// 	{
// 		input: {},
// 		output: { $(,[$("")*])* ;($output:expr $(;)?) $(,[$("")*])* },
// 		args: $args:tt
// 	} => {{
// 		use $crate::writable::WritableInternal;
//
// 		($output).borrow_writable()
// 	}};

	// only one expression
// 	{
// 		input: {},
// 		output: { $(,[$("")*])* ;{ $output:ident $(;)? } $(,[$("")*])* },
// 		output_fields: { { $output_field_name:ident : $output_field_value:expr } }
// 	} => {{
// 		use $crate::writable::WritableInternal;
//
// 		($output_field_value).borrow_writable()
// 	}};

	// combination of writable sources, no captures
	{
		input: {},
		output: { $(internal $internal:tt,)* },
		args: $args:tt
	} => {&{
		struct W;

		impl $crate::writable::Writable for W {
			#[inline]
			fn write_to<W>(&self, w: &mut W) -> Result<(), W::Error>
				where
					W: $crate::write::Write + ?Sized {
				$(
					$crate::fmt_internal_write!(w => $internal);
				)*

				::core::result::Result::Ok(())
			}

			#[inline]
			fn len_hint(&self) -> usize {
				0
				// const CONST_LEN: usize = $crate::fmt_internal_len_hint_literals!(
				// 	$($($output_literals_start,)*)*
				// 	$(
				// 		$($($output_literals,)*)*
				// 	)+
				// );
				// CONST_LEN $(
				// 	+ $crate::fmt_internal_len_hint_value!(self => $output_values)
				// )+
			}
		}

		W
	}};


	// combination of writable sources
	{
		input: {},
		output: { $(internal $internal0:tt,)* $(external { $field_name:ident : $value:expr; [$ty:path] }, $(internal $internal:tt,)*)+ },
		args: $args:tt
	} => {&{
		// for syntax highlighting
		#[allow(unused)]
		{
			$(let $field_name: u8;)*
		}

		#[allow(non_camel_case_types)]
		struct W<'a, $($field_name : $ty + ?Sized),+> {
			$($field_name : &'a $field_name),+
		}

		#[allow(non_camel_case_types)]
		impl<'a, $($field_name : $ty + ?Sized),+> $crate::writable::Writable for W<'a, $($field_name),+> {
			#[inline]
			fn write_to<W>(&self, w: &mut W) -> Result<(), W::Error>
				where
					W: $crate::write::Write + ?Sized {
				$(
					$crate::fmt_internal_write!(w => $internal0);
				)*
				$(
					$crate::fmt_internal_write!(w => { self.$field_name; [$ty] });
					$(
						$crate::fmt_internal_write!(w => $internal);
					)*
				)+

				::core::result::Result::Ok(())
			}

			#[inline]
			fn len_hint(&self) -> usize {
				0
				// const CONST_LEN: usize = $crate::fmt_internal_len_hint_literals!(
				// 	$($($output_literals_start,)*)*
				// 	$(
				// 		$($($output_literals,)*)*
				// 	)+
				// );
				// CONST_LEN $(
				// 	+ $crate::fmt_internal_len_hint_value!(self => $output_values)
				// )+
			}
		}

		// W {
		// 	$($field_name : {
		// 		<_ as $ty>::borrow_writable_internal(($value).borrow())
		// 	}),*
		// }
		W {
			$($field_name : {
				use $ty;
				($value).borrow_writable_internal()
			}),*
		}
	}};
}

#[macro_export]
macro_rules! fmt {
	($($tt:tt)+) => {
		$crate::fmt_internal! {
			input: { $($tt, )+ },
			output: {},
			args: {}
		}
	};
}

#[macro_export]
macro_rules! fmt_struct {
	($value:expr => $name:ident { $($field:ident),* }) => {
		$crate::fmt!([stringify!($name)] " { " $(", " [stringify!($field)] ": " {$field:$value.$field})* "}")
	};
}

#[macro_export]
macro_rules! fmt_tuple_struct {
	($value:expr => $name:ident { $($field:ident),* }) => {
		$crate::fmt!([stringify!($name)] " { " $([stringify!($field)] ": " {$field:$value.$field} ", ")* "}")
	};
}

#[macro_export]
macro_rules! fmt_unit_struct {
	($value:expr => $name:ident { $($field:ident),* }) => {
		$crate::fmt!([stringify!($name)] " { " $([stringify!($field)] ": " {$field:$value.$field} ", ")* "}")
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

    let struct_ = Struct { a: 12, b: true };
    let s = fmt_struct!(struct_ => Struct { a, b });
    let s0 = s.to_string();
    assert_eq!(s0, "Struct { a: 12, b: true, }");

    macro_rules! xyz {
        () => {
            "XYZ"
        };
    }

    let a = "abc";
    let s = fmt!("123" [xyz!()] "abc" {a} "abc");
    let s0 = ToString::to_string(s);
    // assert_eq!(s0.len(), s.len_hint());

    let a = &mut *String::from("abc");
    let s = fmt!("123" [xyz!()] "abc" {a} "abc");
    let s0 = ToString::to_string(s);
    // assert_eq!(s0.len(), s.len_hint());

    let a = String::from("abc");
    let s = fmt!("123" [xyz!()] "abc" {a} "abc");
    let s0 = ToString::to_string(s);
    // assert_eq!(s0.len(), s.len_hint());

    let a = &String::from("abc");
    let s = fmt!("123" [xyz!()] "abc" {a} "abc");
    let s0 = ToString::to_string(s);
    // assert_eq!(s0.len(), s.len_hint());

    let a = &mut String::from("abc");
    let s = fmt!("123" [xyz!()] "abc" {a} "abc");
    let s0 = ToString::to_string(s);
    // assert_eq!(s0.len(), s.len_hint());

    let a: Box<str> = Box::from("abc");
    let s = fmt!("123" [xyz!()] "abc" {a} "abc");
    let s0 = ToString::to_string(s);
    // assert_eq!(s0.len(), s.len_hint());

    let a = &3_i32;
    let s = fmt!("123" [xyz!()] "abc" {a} "abc");
    let s0 = ToString::to_string(s);

    let a = 3_i32;
    let w = fmt!("123" [xyz!()] "abc" {a} "abc");
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
    let s = fmt!("123" [xyz!()] "abc" (&I) "abc" {d:456});
    let s0 = ToString::to_string(s);

    let s = fmt!("123" [xyz!()] "abc" (&const_fn(1, 2) ) "abc");
    // let s = fmt!("123" [xyz!()] "abc" (&const_fn(1, 2) ) "abc");
    let s0 = ToString::to_string(s);
}

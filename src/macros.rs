// macro_rules! fmt_many_internal {
//     () => {};
// }

// macro_rules! fmt_many {
// 	($($l:literal),* $(,)?) => {
// 		concat!($($l),*)
// 	};
//
// 	($($l1:literal, )* $($e:expr $(, $l2:literal)*),* $(,)?) => {
// 		concat!($($l),*)
// 	};
// }

// macro_rules! write_many {
// 	($write:expr $($(, $l1:literal)+)? $(, $e:expr $($(, $l2:literal)+)?)*) => {{
// 		$write $()?
// 	}};
// }

macro_rules! fmt {
	($($l:literal),* $(,)?) => {
		concat!($($l),*)
	};

	($($($l1:literal)?, )* $({ $e:expr } $(, $l2:literal)*),* $(,)?) => {
		$crate::macros::fmt_list!()
	};
}
pub(crate) use fmt;

macro_rules! fmt_list {
	($($l:literal),* $(,)?) => {
		concat!($($l),*)
	};

	($($e:expr),* $(,)?) => {{
		$crate::macros::fmt_list_to_map_internal!($($e),*; a;)
	}};
}
pub(crate) use fmt_list;

macro_rules! fmt_list_to_map_internal {
    ($e1:expr $(, $e2:expr)* $(,)?; $f1:ident; $($f3:ident: $e3:expr),*) => {
    	$crate::macros::fmt_list_to_map_internal!($($e2 ),*; $f1; $f1: $e1 $(, $f3: $e3)*)
    };

    (; $($f1:ident)+; $($f:ident: $e:expr),*) => {
		$crate::macros::fmt_map!{ $($f: $e),* }
	};
}
pub(crate) use fmt_list_to_map_internal;

macro_rules! fmt_map {
    { $($f:ident: $e:expr),* $(,)? } => {{
		struct W< $( $f ),* > {
    		$($f: $f),*
    	}

		W {
			$($f: $e),*
		}
	}};
}
pub(crate) use fmt_map;

fn test() {
    let a = "123";
    let y = fmt!("abc", a);
    y.a;
}

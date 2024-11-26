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
    ($($l:literal)*) => {
    	concat!($($l),*)
    };
    // ($($l1:literal)? $({$e:expr} $($l2:literal)?)*) => {
    //     // $crate::macros::fmt_list!()
    //     {}
    // };
    ($($l1:literal)* $({ $e:expr } $($l2:literal)*)*) => {
		$crate::macros::fmt_list_to_map_internal!(
			name: field_name,
			start_literals: { $($l1)* },
			list: {
				$({ $e } { $($l2)* }),*
			},
			out_map: {}
		)
        // $crate::macros::fmt!(
		// 	$(concat!($($l1),+))?
		// 	$(
		// 		{$e}
		// 		$(concat!($($l2),+))?
		// 	)*
		// )
    };
}
pub(crate) use fmt;

// macro_rules! fmt_list {
// 	($($l:literal),* $(,)?) => {
// 		concat!($($l),*)
// 	};
//
// 	($($l:literal),*$($e:expr),* $(,)?) => {{
// 		$crate::macros::fmt_list_to_map_internal!($($e),*; a;)
// 	}};
// }
// pub(crate) use fmt_list;

macro_rules! fmt_list_to_map_internal {
    (
		name: $name:expr,
		start_literals: { $($start_literal:literal)* },
		list: {
			{ $expr:expr } { $($literal:literal)* }
			$(, { $_expr:expr } { $($_literal:literal)* })*
		},
		out_map: {
			$($out_field:expr => { $out_expr:expr } { $($out_literal:literal)* }),*
		}
	) => {{
		// use $crate::macros::fmt_list_to_map_internal as fmt_list_to_map_internal;
		// use $crate::macros::fmt_list_to_map_internal;
    	// fmt_list_to_map_internal!(
		// $crate::macros::fmt_list_to_map_internal!(
		use $crate::macros::fmt_list_to_map_internal as fmt_list_to_map_internal_;
    	fmt_list_to_map_internal_!(
			name: a$name,
			start_literals: { $($start_literal)* },
			list: {
				$({ $_expr } { $($_literal)* }),*
			},
			out_map: {
				$name => { $expr } { $($literal)* }
				$(, $out_field => { $out_expr } { $($out_literal)* })*
			}
		)
    }};

    (
		name: $name:expr,
		start_literals: { $($start_literal:literal)* },
		list: {},
		out_map: {
			$($out_field:expr => { $out_expr:expr } { $($out_literal:literal)* }),*
		}

	) => {{
		struct W< $($out_field),* > {
    		$($out_field: $out_field),*
    	}

		W {
			$($out_field: $out_expr),*
		}
	}};
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
    let b = "b123b";
    let c = "c123c";
    let y = fmt!("abc" {a} {b} "124" "123");
    // y.a;
}

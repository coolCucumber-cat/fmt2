#![allow(unused_imports, unused_macros)]
#![doc = r#"
SET_STYLE_TYPE = SET_FOREGROUND | SET_BACKGROUND | SET_UNDERLINE

RESET_STYLE_TYPE = RESET_FOREGROUND | RESET_BACKGROUND | RESET_UNDERLINE

ANSI_START = "\x1B["

ANSI_COLOR_END = "m"

ANSI_SET_STYLE = ANSI_START ( RESET_STYLE_TYPE | ( SET_STYLE_TYPE COLOR_CODE ) ) ANSI_COLOR_END
"#]

#[macro_export]
macro_rules! ANSI_START_macro {
    () => {
        "\x1B["
    };
}

#[macro_export]
macro_rules! ansi_color_code_from {
    (black) => {
        "5;0"
    };
    (dark_red) => {
        "5;1"
    };
    (dark_green) => {
        "5;2"
    };
    (dark_yellow) => {
        "5;3"
    };
    (dark_blue) => {
        "5;4"
    };
    (dark_magenta) => {
        "5;5"
    };
    (dark_cyan) => {
        "5;6"
    };
    (grey) => {
        "5;7"
    };
    (dark_grey) => {
        "5;8"
    };
    (red) => {
        "5;9"
    };
    (green) => {
        "5;10"
    };
    (yellow) => {
        "5;11"
    };
    (blue) => {
        "5;12"
    };
    (magenta) => {
        "5;13"
    };
    (cyan) => {
        "5;14"
    };
    (white) => {
        "5;15"
    };
    ({ $r:expr, $g:expr, $b:expr }) => {
        ::core::concat!("2;", $r, ";", $g, ";", $b)
    };
    ($expr:expr) => {
        ::core::concat!("5;", $expr)
    };
    ($ident:ident) => {
        ::core::compile_error!(::core::concat!(
            "unknown color: \"",
            stringify!($ident),
            "\""
        ))
    };
    ($expr:expr) => {
        ::core::compile_error!(::core::concat!(
            "unknown color: \"",
            stringify!($expr),
            "\""
        ))
    };
}

#[macro_export]
macro_rules! ansi_set_single_internal {
	(foreground reset) => {
		"39"
	};
	(background reset) => {
		"49"
	};
	(underline reset) => {
		"59"
	};
	(foreground $c:tt) => {
		::core::concat!("38;", $crate::ansi_color_code_from!($c))
	};
	(background $c:tt) => {
		::core::concat!("48;", $crate::ansi_color_code_from!($c))
	};
	(underline $c:tt) => {
		::core::concat!("58;", $crate::ansi_color_code_from!($c))
	};
	(foreground $cf:tt, background $cb:tt) => {
		::core::concat!(
			$crate::ansi_set_single_internal!(foreground $cf),
			";",
			$crate::ansi_set_single_internal!(background $cb),
		)
	};
}

#[macro_export]
macro_rules! ansi_set_single {
	($($tt:tt)*) => {
		::core::concat!($crate::ANSI_START_macro!(), $crate::ansi_set_single_internal!($($tt)*), "m")
	};
}

#[macro_export]
macro_rules! ansi_set {
	(foreground $f:tt) => {
		$crate::ansi_set_single!(foreground $f)
	};
	(background $b:tt) => {
		$crate::ansi_set_single!(background $b)
	};
	(underline $u:tt) => {
		$crate::ansi_set_single!(underline $u)
	};
	(foreground $f:tt, background $b:tt) => {
		$crate::ansi_set_single!(foreground $f, background $b)
	};
	(background $b:tt, underline $u:tt) => {
		::core::concat!(
			$crate::ansi_set_single!(background $b),
			$crate::ansi_set_single!(underline $u),
		)
	};
	(foreground $f:tt, underline $u:tt) => {
		::core::concat!(
			$crate::ansi_set_single!(foreground $f),
			$crate::ansi_set_single!(underline $u),
		)
	};
	(foreground $f:tt, background $b:tt, underline $u:tt) => {
		::core::concat!(
			$crate::ansi_set_single!(foreground $f, background $b),
			$crate::ansi_set_single!(underline $u),
		)
	};
}

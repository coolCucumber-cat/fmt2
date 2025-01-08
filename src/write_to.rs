use crate::write::Write;

pub trait WriteTo {
    const ENDS_IN_NEWLINE: bool = false;

    fn write_to<W>(&self, w: &mut W) -> Result<(), W::Error>
    where
        W: Write + ?Sized;

    #[inline]
    fn len_hint(&self) -> usize {
        0
    }
}

pub trait Fmt {
    fn fmt(&self) -> &(impl WriteTo + ?Sized);
}

impl<T> Fmt for T
where
    T: WriteTo + ?Sized,
{
    #[inline]
    fn fmt(&self) -> &(impl WriteTo + ?Sized) {
        self
    }
}

#[macro_export]
macro_rules! declare_fmt_wrapper_struct {
    { $Struct:ident $Trait:ident $fn:ident $(, $($($rest:tt)+)?)? } => {
        pub struct $Struct<T>(T)
        where
            T: ?Sized,
            Self: $crate::write_to::WriteTo;

        pub trait $Trait {
            fn $fn(&self) -> &(impl $crate::write_to::WriteTo + ?Sized);
        }

        impl<T> $Trait for T
        where
            T: ?Sized,
            $Struct<T>: $crate::write_to::WriteTo,
        {
            #[inline]
            fn $fn(&self) -> &(impl $crate::write_to::WriteTo + ?Sized) {
                unsafe { &*(self as *const Self as *const $Struct<Self>) }
            }
        }

        $($($crate::declare_fmt_wrapper_struct! { $($rest)+ })?)?
    };
    { $Struct:ident <$(const $CONST:ident : $ConstType:ty),* $(,)?> $Trait:ident $fn:ident $(, $($($rest:tt)+)?)? } => {
        pub struct $Struct<T $(, const $CONST: $ConstType)*>(T)
        where
            T: ?Sized,
            $Struct<T $(, { $CONST })*>: $crate::write_to::WriteTo;

        pub trait $Trait {
            fn $fn<$(const $CONST : $ConstType),*>(&self) -> &$Struct<Self $(, { $CONST })*>
            where
                $Struct<Self $(, { $CONST })*>: $crate::write_to::WriteTo;
        }

        impl<T> $Trait for T
        where
            T: ?Sized,
        {
            #[inline]
            fn $fn<$(const $CONST : $ConstType),*>(&self) -> &$Struct<Self $(, { $CONST })*>
            where
                $Struct<Self $(, { $CONST })*>: $crate::write_to::WriteTo
            {
                unsafe { &*(self as *const Self as *const $Struct<Self $(, $CONST)*>) }
            }
        }
//         pub struct $Struct<T $($(, const $CONST: $ConstType)*)?>(T)
//         where
//             T: ?Sized,
//             Self: $crate::write_to::WriteTo;
//
//         pub trait $Trait $(<$(const $CONST: $ConstType),*>)? {
//             fn $fn(&self) -> &(impl $crate::write_to::WriteTo + ?Sized);
//         }
//
//         impl<T $($(, const $CONST: $ConstType)*)?> $Trait $(<$($CONST),*>)? for T
//             where
//             T: ?Sized,
//             $Struct<T $($(, { $CONST })*)?>: $crate::write_to::WriteTo,
//         {
//             #[inline]
//             fn $fn(&self) -> &(impl $crate::write_to::WriteTo + ?Sized) {
//                 unsafe { &*(self as *const Self as *const $Struct<Self $($(, $CONST)*)?>) }
//             }
//         }

        $($($crate::declare_fmt_wrapper_struct! { $($rest)+ })?)?

    };
}

macro_rules! declare_std_write_to_wrapper_struct_internal {
    { $($Struct:ident $(<$(const $CONST:ident : $ConstType:ty),* $(,)?>)? $Trait:ident $fn:ident => $StdTrait:ident $write_fn:ident),* $(,)? } => {
        $(
            $crate::declare_fmt_wrapper_struct! { $Struct $(<$(const $CONST : $ConstType),*>)? $Trait $fn }

            impl<T $($(, const $CONST : $ConstType)*)?> $crate::write_to::WriteTo for $Struct<T $($(, { $CONST })*)?>
            where
                T: ::core::fmt::$StdTrait + ?Sized,
            {
                #[inline]
                fn write_to<W>(&self, w: &mut W) -> ::core::result::Result<(), W::Error>
                where
                    W: $crate::write::Write + ?Sized,
                {
                    $crate::write::Write::$write_fn::<T $($(, { $CONST })*)?>(w, &self.0)
                }
            }
        )*
    };
}

macro_rules! impl_fmt_trait_internal {
    { $StdTrait:ident $std_fn:ident => $Trait:ident $fn:ident => $($ty:ty)*} => {
        $(
            impl $Trait for $ty {
                #[inline]
                fn $fn(&self) -> &(impl $crate::write_to::WriteTo + ?Sized) {
                    $StdTrait::$std_fn(self)
                }
            }
        )*
    };
}

declare_fmt_wrapper_struct! {
    Debug   FmtDebug    fmt_debug,
    Binary  FmtBinary   fmt_binary,
    Octal   FmtOctal    fmt_octal,
    Hex     FmtHex      fmt_hex,
    Precision<const PRECISION: u8> FmtPrecision fmt_precision,
}

declare_std_write_to_wrapper_struct_internal! {
    StdDisplay  FmtStdDisplay   fmt_std_display => Display  write_std_display,
    StdDebug    FmtStdDebug     fmt_std_debug   => Debug    write_std_debug,
    StdBinary   FmtStdBinary    fmt_std_binary  => Binary   write_std_binary,
    StdOctal    FmtStdOctal     fmt_std_octal   => Octal    write_std_octal,
    StdHex      FmtStdHex       fmt_std_hex     => UpperHex write_std_upper_hex,
    StdPrecision<const PRECISION: u8>   FmtStdPrecision fmt_std_precision   => Display write_std_precision,
}

impl_fmt_trait_internal! { FmtStdDisplay    fmt_std_display => Fmt          fmt         => u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 f32 f64 }
impl_fmt_trait_internal! { FmtStdDebug      fmt_std_debug   => FmtDebug     fmt_debug   => u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 f32 f64 }
impl_fmt_trait_internal! { FmtStdBinary     fmt_std_binary  => FmtBinary    fmt_binary  => u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 }
impl_fmt_trait_internal! { FmtStdOctal      fmt_std_octal   => FmtOctal     fmt_octal   => u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 }
impl_fmt_trait_internal! { FmtStdHex        fmt_std_hex     => FmtHex       fmt_hex     => u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 }

impl WriteTo for core::fmt::Arguments<'_> {
    fn write_to<W>(&self, w: &mut W) -> Result<(), W::Error>
    where
        W: Write + ?Sized,
    {
        w.write_std_args_ref(self)
    }
}

impl WriteTo for Debug<str> {
    fn write_to<W>(&self, w: &mut W) -> Result<(), W::Error>
    where
        W: Write + ?Sized,
    {
        w.write_str(&self.0)
    }
}

// impl FmtDebug for bool {
//     type Target = Self;
//     fn fmt_debug(&self) -> &Self::Target {
//         self
//     }
// }

// pub trait WriteToFor<W>
// where
//     W: crate::write::Write + ?Sized,
// {
//     fn write_to_for(&self, w: &mut W) -> Result<(), W::Error>;
// }
//
// impl<F, W> WriteToFor<W> for F
// where
//     F: Fn(&mut W) -> Result<(), W::Error>,
//     W: crate::write::Write,
// {
//     fn write_to_for(&self, w: &mut W) -> Result<(), W::Error> {
//         self(w)
//     }
// }

pub trait ToString {
    fn to_string(&self) -> String;
}

impl<T> ToString for T
where
    T: Fmt + ?Sized,
{
    fn to_string(&self) -> String {
        let wt = self.fmt();
        let mut s = String::with_capacity(wt.len_hint());
        s.write(wt).into_ok();
        s
    }
}

#[macro_export]
macro_rules! impl_write_to_for_std_display {
	{ $($ty:ident $write_to_fn:ident),* $(,)? } => {
		$(
			impl $crate::write_to::WriteTo for $name {
				#[inline]
				fn write_to<W>(&self, w: &mut W) -> Result<(), W::Error>
					where
						W: $crate::write::Write + ?Sized {
					$crate::write::Write::write_std_display(w, self)
				}
			}
		)*
	};
}

#[macro_export]
macro_rules! impl_std_display_for_write_to {
	{ $($name:ty),* $(,)? } => {
		$(
			impl ::core::fmt::Display for $name {
				#[inline]
				fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
					$crate::write::Write::write(f, self)
				}
			}
		)*
	};
}

#[allow(
    clippy::allow_attributes,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::missing_const_for_fn,
    unused_variables,
    unused_imports
)]
#[cfg(test)]
mod tests {
    use core::borrow::Borrow;

    use crate::{
        str::{StaticStr, Str},
        write_to::WriteTo,
    };

    use super::ToString;

    #[test]
    fn borrow() {
        let k = {
            let s = "123";
            // let s = String::new();
            s.str()
        };
        ToString::to_string(k);
        let k = {
            let s = true;
            s.static_str()
        };
        ToString::to_string(k);
    }

    #[test]
    fn write() {
        use crate::{
            write::{Flush, Write, WriteInfallible},
            write_to::WriteTo,
        };

        struct Test(bool);

        impl WriteTo for Test {
            fn write_to<W>(&self, w: &mut W) -> Result<(), W::Error>
            where
                W: Write + ?Sized,
            {
                w.write_str(self.0.str())
            }
        }

        let mut s = String::new();
        s.write(&Test(true)).into_ok();
        assert_eq!(s, "true");

        let mut s = String::new();
        s.write(&Test(false)).into_ok();
        assert_eq!(s, "false");

        let mut s = String::new();
        Test(true).write_to(&mut s).into_ok();
        assert_eq!(s, "true");

        let mut s = String::new();
        Test(false).write_to(&mut s).into_ok();
        assert_eq!(s, "false");

        let mut s = String::new();
        s.write_str_infallible("123456");
        assert_eq!(s, "123456");

        let mut s = String::new();
        WriteInfallible::write_str_infallible(&mut s, "123abc");
        assert_eq!(s, "123abc");

        let mut s = String::new();
        "123abc".write_to(&mut s).into_ok();
        assert_eq!(s, "123abc");
    }
}

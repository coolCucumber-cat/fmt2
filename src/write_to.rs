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

pub trait FmtAdvanced {
    type Target: WriteTo + ?Sized;
    fn fmt_advanced(&self) -> &(impl WriteTo + ?Sized);
}

impl<T> FmtAdvanced for T
where
    T: WriteTo + ?Sized,
{
    type Target = Self;
    #[inline]
    fn fmt_advanced(&self) -> &Self::Target {
        self
    }
}

// pub trait FmtAdvancedWithTarget<W>
// where
//     W: Write + ?Sized,
// {
//     type Target: WriteToFor<W> + ?Sized;
//     fn fmt_advanced_with_target(&self) -> &Self::Target;
// }
//
// impl<W, T> FmtAdvancedWithTarget<W> for T
// where
//     T: WriteTo + ?Sized,
//     W: Write + ?Sized,
// {
//     type Target = Self;
//     #[inline]
//     fn fmt_advanced_with_target(&self) -> &Self::Target {
//         self
//     }
// }

macro_rules! declare_write_to_wrapper_struct_internal {
    { $($Struct:ident $(<$(const $CONST:ident : $ConstType:ty),* $(,)?>)? $Trait:ident $fn:ident $TraitAdvanced:ident $fn_advanced:ident),* $(,)? } => {
        $(
            pub struct $Struct<T $($(, const $CONST: $ConstType)*)?>(T)
            where
                T: ?Sized,
                Self: $crate::write_to::WriteTo;

            pub trait $Trait $(<$(const $CONST: $ConstType),*>)? {
                fn $fn(&self) -> &(impl $crate::write_to::WriteTo + ?Sized);
            }

            impl<T $($(, const $CONST: $ConstType)*)?> $Trait $(<$($CONST),*>)? for T
                where
                T: ?Sized,
                $Struct<T $($(, { $CONST })*)?>: $crate::write_to::WriteTo,
            {
                #[inline]
                fn $fn(&self) -> &(impl $crate::write_to::WriteTo + ?Sized) {
                    unsafe { &*(self as *const Self as *const $Struct<Self $($(, $CONST)*)?>) }
                }
            }

            pub trait $TraitAdvanced $(<$(const $CONST: $ConstType),*>)? {
                type Target: $crate::write_to::WriteTo + ?Sized;
                fn $fn_advanced(&self) -> &Self::Target;
            }

            impl<T $($(, const $CONST: $ConstType)*)?> $TraitAdvanced $(<$($CONST),*>)? for T
            where
                T: $Trait $(<$({ $CONST }),*>)? + ?Sized,
                // T: ?Sized,
                // $Struct<T $($(, { $CONST })*)?>: $Trait,
            {
                type Target = impl $crate::write_to::WriteTo + ?Sized;
                #[inline]
                fn $fn_advanced(&self) -> &Self::Target {
                    $Trait $(::<$({ $CONST }),*>)? ::$fn(self)
                }
            }
        )*
    };
}

macro_rules! declare_std_write_to_wrapper_struct_internal {
    { $($Struct:ident $Trait:ident $fn:ident $TraitAdvanced:ident $fn_advanced:ident => $StdTrait:ident $write_fn:ident),* $(,)? } => {
        $(
            declare_write_to_wrapper_struct_internal! { $Struct $Trait $fn $TraitAdvanced $fn_advanced }

            impl<T> $crate::write_to::WriteTo for $Struct<T>
            where
                T: ::core::fmt::$StdTrait + ?Sized,
            {
                #[inline]
                fn write_to<W>(&self, w: &mut W) -> ::core::result::Result<(), W::Error>
                where
                    W: $crate::write::Write + ?Sized,
                {
                    $crate::write::Write::$write_fn(w, &self.0)
                }
            }
        )*
    };
}

macro_rules! impl_fmt_trait_internal {
    { $StdFmtTrait:ident $std_fmt_fn:ident => $FmtTrait:ident $fmt_fn:ident => $($ty:ty)*} => {
        $(
            impl $FmtTrait for $ty {
                #[inline]
                fn $fmt_fn(&self) -> &(impl $crate::write_to::WriteTo + ?Sized) {
                    $StdFmtTrait::$std_fmt_fn(self)
                }
            }
        )*
    };
}

declare_write_to_wrapper_struct_internal! {
    Debug   FmtDebug    fmt_debug FmtDebugAdvanced    fmt_debug_advanced,
    Binary  FmtBinary   fmt_binary FmtBinaryAdvanced   fmt_binary_advanced,
    Octal   FmtOctal    fmt_octal FmtOctalAdvanced    fmt_octal_advanced,
    Hex     FmtHex      fmt_hex FmtHexAdvanced      fmt_hex_advanced,
    Precision<const PRECISION: u8> FmtPrecision fmt_precision FmtPrecisionAdvanced fmt_precision_advanced,
}

// pub trait FmtPrecisionInternal<const PRECISION: u8>: FmtPrecision<PRECISION> {
//     fn fmt_precision_internal(&self) -> &Self;
// }
//
// impl<T, const PRECISION: u8> FmtPrecisionInternal<PRECISION> for T
// where
//     T: FmtPrecision<PRECISION>,
// {
//     #[inline]
//     fn fmt_precision_internal(&self) -> &Self {
//         self
//     }
// }

declare_std_write_to_wrapper_struct_internal! {
    StdDisplay  FmtStdDisplay   fmt_std_display FmtStdDisplayAdvanced   fmt_std_display_advanced => Display  write_std_display,
    StdDebug    FmtStdDebug     fmt_std_debug   FmtStdDebugAdvanced     fmt_std_debug_advanced   => Debug    write_std_debug,
    StdBinary   FmtStdBinary    fmt_std_binary  FmtStdBinaryAdvanced    fmt_std_binary_advanced  => Binary   write_std_binary,
    StdOctal    FmtStdOctal     fmt_std_octal   FmtStdOctalAdvanced     fmt_std_octal_advanced   => Octal    write_std_octal,
    StdHex      FmtStdHex       fmt_std_hex     FmtStdHexAdvanced       fmt_std_hex_advanced     => UpperHex write_std_upper_hex,
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

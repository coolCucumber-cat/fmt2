use core::ops::Deref;

use crate::write::Write;

pub trait WriteTo {
    fn write_to<W>(&self, w: &mut W) -> Result<(), W::Error>
    where
        W: Write + ?Sized;

    #[inline]
    fn len_hint(&self) -> usize {
        0
    }
}

pub trait Fmt {
    type Target: crate::write_to::WriteTo + ?Sized;

    fn fmt(&self) -> &Self::Target;
}

pub trait Fmt2 {
    fn fmt(&self) -> &impl WriteTo;
}

impl<T> Fmt for T
where
    T: WriteTo + ?Sized,
{
    type Target = Self;
    #[inline]
    fn fmt(&self) -> &Self::Target {
        self
    }
}

#[macro_export]
macro_rules! declare_WriteTo_wrapper_struct_internal {
    { $($Struct:ident $Trait:ident $fn:ident),* $(,)? } => {
        $(
            pub struct $Struct<T>(T)
            where
                T: ?Sized,
                Self: $crate::write_to::WriteTo;

            pub trait $Trait {
                type Target: $crate::write_to::WriteTo + ?Sized;
                fn $fn(&self) -> &Self::Target;
            }

            impl<T> $Trait for T
            where
                T: ?Sized,
                $Struct<T>: $crate::write_to::WriteTo,
            {
                type Target = $Struct<Self>;
                #[inline]
                fn $fn(&self) -> &Self::Target {
                    unsafe { &*(self as *const Self as *const $Struct<Self>) }
                }
            }
        )*
    };
}

macro_rules! declare_std_WriteTo_wrapper_struct_internal {
    { $($Struct:ident $Trait:ident $fn:ident => $StdTrait:ident $write_fn:ident),* $(,)? } => {
        $(
            $crate::declare_WriteTo_wrapper_struct_internal! { $Struct $Trait $fn }

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
                type Target = <Self as $StdFmtTrait>::Target;
                #[inline]
                fn $fmt_fn(&self) -> &Self::Target {
                    $StdFmtTrait::$std_fmt_fn(self)
                }
            }
        )*
    };
}

declare_WriteTo_wrapper_struct_internal! {
    Debug   FmtDebug    fmt_debug,
    Binary  FmtBinary   fmt_binary,
    Octal   FmtOctal    fmt_octal,
    Hex     FmtHex      fmt_hex,
}

declare_std_WriteTo_wrapper_struct_internal! {
    StdDisplay  FmtStdDisplay   fmt_std_display => Display  write_std_display,
    StdDebug    FmtStdDebug     fmt_std_debug   => Debug    write_std_debug,
    StdBinary   FmtStdBinary    fmt_std_binary  => Binary   write_std_binary,
    StdOctal    FmtStdOctal     fmt_std_octal   => Octal    write_std_octal,
    StdHex      FmtStdHex       fmt_std_hex     => UpperHex write_std_upper_hex,
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

impl FmtDebug for bool {
    type Target = Self;
    fn fmt_debug(&self) -> &Self::Target {
        self
    }
}

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

pub trait ConstStr {
    const CONST_STR: &'static str;
}

pub trait StaticStrAndStr {
    fn static_str_and_str(&self) -> &'static str;
}

impl<T> StaticStrAndStr for T
where
    T: ConstStr + ?Sized,
{
    #[inline]
    fn static_str_and_str(&self) -> &'static str {
        Self::CONST_STR
    }
}

pub trait StaticStrInternal {
    fn static_str_internal(self) -> &'static str;
}

impl<T> StaticStrInternal for &T
where
    T: StaticStrAndStr + ?Sized,
{
    #[inline]
    fn static_str_internal(self) -> &'static str {
        self.static_str_and_str()
    }
}

impl StaticStrInternal for &'static str {
    fn static_str_internal(self) -> &'static str {
        self
    }
}

pub trait StaticStr<'a> {
    fn static_str(&'a self) -> &'static str;
}

impl<'a, T> StaticStr<'a> for T
where
    T: 'a,
    &'a T: StaticStrInternal,
{
    #[inline]
    fn static_str(&'a self) -> &'static str {
        StaticStrInternal::static_str_internal(self)
    }
}

pub trait Str {
    fn str(&self) -> &str;
}

impl<T> Str for T
where
    T: StaticStrAndStr + ?Sized,
{
    #[inline]
    fn str(&self) -> &str {
        self.static_str_and_str()
    }
}

impl<T> WriteTo for T
where
    T: Str + ?Sized,
{
    #[inline]
    fn write_to<W>(&self, w: &mut W) -> Result<(), W::Error>
    where
        W: Write + ?Sized,
    {
        w.write_str(self.str())
    }

    #[inline]
    fn len_hint(&self) -> usize {
        self.str().len()
    }
}

impl Str for str {
    #[inline]
    fn str(&self) -> &str {
        self
    }
}

impl StaticStrAndStr for bool {
    fn static_str_and_str(&self) -> &'static str {
        if *self {
            "true"
        } else {
            "false"
        }
    }
}

pub trait ToString {
    fn to_string(&self) -> String;
}

impl<T> ToString for T
where
    T: WriteTo + ?Sized,
{
    fn to_string(&self) -> String {
        let mut s = String::with_capacity(self.len_hint());
        s.write(self).into_ok();
        s
    }
}

#[derive(Clone, Copy)]
pub struct WithStr<T, U = &'static str> {
    pub value: T,
    pub str: U,
}

impl<T, U> WithStr<T, U> {
    pub const fn new(value: T, str: U) -> Self {
        Self { value, str }
    }

    pub fn map_value<V>(self, f: impl FnOnce(T) -> V) -> WithStr<V, U> {
        WithStr {
            value: f(self.value),
            str: self.str,
        }
    }

    pub fn replace_value<V>(self, value: V) -> WithStr<V, U> {
        WithStr {
            value,
            str: self.str,
        }
    }
}

impl<T, U> Str for WithStr<T, &'_ U>
where
    U: Str + ?Sized,
{
    fn str(&self) -> &str {
        self.str.str()
    }
}

impl<T0, T1, U> AsRef<T1> for WithStr<T0, U>
where
    T0: AsRef<T1>,
    T1: ?Sized,
{
    #[inline]
    fn as_ref(&self) -> &T1 {
        self.value.as_ref()
    }
}

impl<T, U> Deref for WithStr<T, U> {
    type Target = Self;

    fn deref(&self) -> &Self::Target {
        self
    }
}

#[macro_export]
macro_rules! impl_WriteTo_const_str_for {
	{ $($name:path $(=> $value:expr)?),* $(,)? } => {
		$(
			/// also implements [`WriteToStr`] and [`WriteToStaticStr`]
			impl $crate::write_to::WriteToConstStr for $name {
				const CONST_STR: &str = $crate::default_token!($($value)?, stringify!($name));
			}
		)*
	};
}

#[macro_export]
macro_rules! impl_WriteTo_for_std_display {
	{ $($ty:ident $WriteTo_fn:ident),* $(,)? } => {
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
macro_rules! impl_display_for_WriteTo {
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

#[macro_export]
macro_rules! impl_display_for_WriteTo_str {
	{ $($name:ty),* $(,)? } => {
		$(
			impl ::core::fmt::Display for $name {
				#[inline]
				fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
					::core::fmt::Formatter::write_str(f, $crate::write_to::WriteToStr::str(self))
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

    use crate::write_to::{StaticStr, Str, WriteTo};

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
            write_to::{Str, WriteTo},
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

        struct HasNoCustomFlush;

        impl Write for HasNoCustomFlush {
            type Error = u32;

            fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
                Ok(())
            }
        }

        struct HasCustomFlush;

        impl Write for HasCustomFlush {
            type Error = u32;

            fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
                Ok(())
            }
        }

        impl Flush for HasCustomFlush {
            type Error = &'static str;

            fn flush(&mut self) -> Result<(), Self::Error> {
                Err("flushed")
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

        let mut stdout = std::io::stdout();
        stdout.write("abc").unwrap();
        // WriteFlush::flush(&mut stdout).unwrap();
        stdout.flush().unwrap();

        // assert_eq!(HasCustomFlush.flush_hint(), Err("flushes"));
        // assert_eq!(HasNoCustomFlush.flush_hint(), Ok(()));
    }
}

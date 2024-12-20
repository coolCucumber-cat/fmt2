use core::ops::Deref;

use crate::write::Write;

pub trait Writable {
    fn write_to<W>(&self, w: &mut W) -> Result<(), W::Error>
    where
        W: Write + ?Sized;

    #[inline]
    fn len_hint(&self) -> usize {
        0
    }
}

pub trait AsWritable: Writable {
    fn as_writable(&self) -> &Self;
}

impl<T> AsWritable for T
where
    T: Writable + ?Sized,
{
    #[inline]
    fn as_writable(&self) -> &Self {
        self
    }
}

#[macro_export]
macro_rules! declare_writable_wrapper_struct {
    { $($Struct:ident $Trait:ident $fn:ident),* $(,)? } => {
        $(
            pub struct $Struct<T>(T)
            where
                T: ?Sized,
                Self: $crate::writable::Writable;

            pub trait $Trait {
                type Target: $crate::writable::Writable + ?Sized;
                fn $fn(&self) -> &Self::Target;
            }

            impl<T> $Trait for T
            where
                T: ?Sized,
                $Struct<T>: $crate::writable::Writable,
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

macro_rules! declare_std_writable_wrapper_struct {
    { $($Struct:ident $Trait:ident $fn:ident => $StdTrait:ident $write_fn:ident => { $($ty:ty)* }),* $(,)? } => {
        $(
            $crate::declare_writable_wrapper_struct! { $Struct $Trait $fn }

            impl<T> $crate::writable::Writable for $Struct<T>
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

            $(
                impl $crate::writable::Writable for $Struct<$ty>
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
        )*
    };
}

macro_rules! declare_std_writable_wrapper_struct_2 {
    { $($display_debug:ty)*, $($int:ty)* } => {
        declare_std_writable_wrapper_struct! {
            StdDisplay AsStdDisplay as_std_display => Display write_std_display => { $($display_debug)* $($int)* },
            StdDebug AsStdDebug as_std_debug => Debug write_std_debug => { $($display_debug)* $($int)* },
            StdBinary AsStdBinary as_std_binary => Binary write_std_binary => { $($int)* },
            StdOctal AsStdOctal as_std_octal => Octal write_std_octal => { $($int)* },
            StdHex AsStdHex as_std_hex => UpperHex write_std_hex => { $($int)* },
        }
    };
}

declare_writable_wrapper_struct! {
    Debug AsDebug as_debug,
    Binary AsBinary as_binary,
    Octal AsOctal as_octal,
    Hex AsHex as_hex,
    StdArguments AsStdArguments as_std_arguments,
}

declare_std_writable_wrapper_struct_2! { f32 f64, u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 }

pub trait WritableDebug {
    fn write_to_debug<W>(&self, w: &mut W) -> Result<(), W::Error>
    where
        W: Write + ?Sized;

    #[inline]
    fn len_hint(&self) -> usize {
        0
    }
}

pub trait WritableBinary {
    fn write_to_binary<W>(&self, w: &mut W) -> Result<(), W::Error>
    where
        W: Write + ?Sized;

    #[inline]
    fn len_hint(&self) -> usize {
        0
    }
}

pub trait WritableHexadecimal {
    fn write_to_hexadecimal<W>(&self, w: &mut W) -> Result<(), W::Error>
    where
        W: Write + ?Sized;

    #[inline]
    fn len_hint(&self) -> usize {
        0
    }
}

pub trait WritableInternal: Writable {
    fn write_to_internal<W>(&self, w: &mut W) -> Result<(), W::Error>
    where
        W: Write + ?Sized;
    fn len_hint_internal(&self) -> usize;
    fn borrow_writable_internal(&self) -> &Self;
}
impl<T> WritableInternal for T
where
    T: Writable + ?Sized,
{
    fn write_to_internal<W>(&self, w: &mut W) -> Result<(), W::Error>
    where
        W: Write + ?Sized,
    {
        self.write_to(w)
    }
    #[inline]
    fn len_hint_internal(&self) -> usize {
        self.len_hint()
    }
    #[inline]
    fn borrow_writable_internal(&self) -> &Self {
        self
    }
}
pub trait WritableDebugInternal: WritableDebug {
    fn write_to_internal<W>(&self, w: &mut W) -> Result<(), W::Error>
    where
        W: Write + ?Sized;
    fn len_hint_internal(&self) -> usize;
    fn borrow_writable_internal(&self) -> &Self;
}
impl<T> WritableDebugInternal for T
where
    T: WritableDebug + ?Sized,
{
    fn write_to_internal<W>(&self, w: &mut W) -> Result<(), W::Error>
    where
        W: Write + ?Sized,
    {
        self.write_to_debug(w)
    }
    #[inline]
    fn len_hint_internal(&self) -> usize {
        self.len_hint()
    }
    #[inline]
    fn borrow_writable_internal(&self) -> &Self {
        self
    }
}
pub trait WritableBinaryInternal: WritableBinary {
    fn write_to_internal<W>(&self, w: &mut W) -> Result<(), W::Error>
    where
        W: Write + ?Sized;
    fn len_hint_internal(&self) -> usize;
    fn borrow_writable_internal(&self) -> &Self;
}
impl<T> WritableBinaryInternal for T
where
    T: WritableBinary + ?Sized,
{
    fn write_to_internal<W>(&self, w: &mut W) -> Result<(), W::Error>
    where
        W: Write + ?Sized,
    {
        self.write_to_binary(w)
    }
    #[inline]
    fn len_hint_internal(&self) -> usize {
        self.len_hint()
    }
    #[inline]
    fn borrow_writable_internal(&self) -> &Self {
        self
    }
}
pub trait WritableHexadecimalInternal: WritableHexadecimal {
    fn write_to_internal<W>(&self, w: &mut W) -> Result<(), W::Error>
    where
        W: Write + ?Sized;
    fn len_hint_internal(&self) -> usize;

    fn borrow_writable_internal(&self) -> &Self;
}
impl<T> WritableHexadecimalInternal for T
where
    T: WritableHexadecimal + ?Sized,
{
    fn write_to_internal<W>(&self, w: &mut W) -> Result<(), W::Error>
    where
        W: Write + ?Sized,
    {
        self.write_to_hexadecimal(w)
    }
    #[inline]
    fn len_hint_internal(&self) -> usize {
        self.len_hint()
    }
    #[inline]
    fn borrow_writable_internal(&self) -> &Self {
        self
    }
}

impl Writable for Debug<str> {
    fn write_to<W>(&self, w: &mut W) -> Result<(), W::Error>
    where
        W: Write + ?Sized,
    {
        w.write_str(&self.0)
    }
}

impl AsDebug for bool {
    type Target = Self;
    fn as_debug(&self) -> &Self::Target {
        self
    }
}

// pub trait WritableFor<W>
// where
//     W: crate::write::Write + ?Sized,
// {
//     fn write_to_for(&self, w: &mut W) -> Result<(), W::Error>;
// }
//
// impl<F, W> WritableFor<W> for F
// where
//     F: Fn(&mut W) -> Result<(), W::Error>,
//     W: crate::write::Write,
// {
//     fn write_to_for(&self, w: &mut W) -> Result<(), W::Error> {
//         self(w)
//     }
// }

pub trait WritableConstStr {
    const CONST_STR: &'static str;
}

pub trait WritableStaticStrAndStr {
    fn static_str_and_str(&self) -> &'static str;
}

impl<T> WritableStaticStrAndStr for T
where
    T: WritableConstStr + ?Sized,
{
    #[inline]
    fn static_str_and_str(&self) -> &'static str {
        Self::CONST_STR
    }
}

pub trait WritableStaticStrInternal {
    fn static_str_internal(self) -> &'static str;
}

impl<T> WritableStaticStrInternal for &T
where
    T: WritableStaticStrAndStr + ?Sized,
{
    #[inline]
    fn static_str_internal(self) -> &'static str {
        self.static_str_and_str()
    }
}

impl WritableStaticStrInternal for &'static str {
    fn static_str_internal(self) -> &'static str {
        self
    }
}

pub trait WritableStaticStr<'a> {
    fn static_str(&'a self) -> &'static str;
}

impl<'a, T> WritableStaticStr<'a> for T
where
    T: 'a,
    &'a T: WritableStaticStrInternal,
{
    #[inline]
    fn static_str(&'a self) -> &'static str {
        WritableStaticStrInternal::static_str_internal(self)
    }
}

pub trait WritableStr {
    fn str(&self) -> &str;
}

impl<T> WritableStr for T
where
    T: WritableStaticStrAndStr + ?Sized,
{
    #[inline]
    fn str(&self) -> &str {
        self.static_str_and_str()
    }
}

impl<T> Writable for T
where
    T: WritableStr + ?Sized,
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

impl WritableStr for str {
    #[inline]
    fn str(&self) -> &str {
        self
    }
}

impl WritableStaticStrAndStr for bool {
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
    T: Writable + ?Sized,
{
    fn to_string(&self) -> String {
        let mut s = String::with_capacity(self.len_hint());
        s.write(self).into_ok();
        s
    }
}

#[derive(Clone, Copy)]
pub struct WithWritableStr<T, U = &'static str> {
    pub value: T,
    pub writable: U,
}

impl<T, U> WithWritableStr<T, U> {
    pub const fn new(value: T, displayable: U) -> Self {
        Self {
            value,
            writable: displayable,
        }
    }

    pub fn map_value<V>(self, f: impl FnOnce(T) -> V) -> WithWritableStr<V, U> {
        WithWritableStr {
            value: f(self.value),
            writable: self.writable,
        }
    }

    pub fn replace_value<V>(self, value: V) -> WithWritableStr<V, U> {
        WithWritableStr {
            value,
            writable: self.writable,
        }
    }
}

impl<T, U> WritableStr for WithWritableStr<T, &'_ U>
where
    U: WritableStr + ?Sized,
{
    fn str(&self) -> &str {
        self.writable.str()
    }
}

impl<T0, T1, U> AsRef<T1> for WithWritableStr<T0, U>
where
    T0: AsRef<T1>,
    T1: ?Sized,
{
    #[inline]
    fn as_ref(&self) -> &T1 {
        self.value.as_ref()
    }
}

impl<T, U> Deref for WithWritableStr<T, U> {
    type Target = Self;

    fn deref(&self) -> &Self::Target {
        self
    }
}

#[macro_export]
macro_rules! impl_writable_const_str_for {
	{ $($name:path $(=> $value:expr)?),* $(,)? } => {
		$(
			/// also implements [`WritableStr`] and [`WritableStaticStr`]
			impl ::fmt2::writable::WritableConstStr for $name {
				const CONST_STR: &str = ::fmt2::default_token!($($value)?, stringify!($name));
			}
		)*
	};
}

#[macro_export]
macro_rules! impl_writable_for_std_display {
	{ $($ty:ident $writable_fn:ident),* $(,)? } => {
		$(
			impl $crate::writable::Writable for $name {
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
macro_rules! impl_display_for_writable {
	{ $($name:ty),* $(,)? } => {
		$(
			impl ::core::fmt::Display for $name {
				#[inline]
				fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
					::fmt2::write::Write::write(f, self)
				}
			}
		)*
	};
}

#[macro_export]
macro_rules! impl_display_for_writable_str {
	{ $($name:ty),* $(,)? } => {
		$(
			impl ::core::fmt::Display for $name {
				#[inline]
				fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
					::core::fmt::Formatter::write_str(f, ::fmt2::writable::WritableStr::str(self))
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

    use crate::writable::{Writable, WritableDebug, WritableStaticStr, WritableStr};

    use super::ToString;

    #[test]
    fn borrow() {
        use super::WritableInternal;

        let i = 32_i32;

        let i0 = i.borrow_writable_internal();

        let i = &32;
        let i0 = i.borrow_writable_internal();

        let i = &mut 32;
        let i0 = i.borrow_writable_internal();

        let s = "123";
        let s0 = s.borrow_writable_internal();

        let s = &mut *String::new();
        let s0 = s.borrow_writable_internal();

        let s = String::new();
        let s0 = s.borrow_writable_internal();

        let s = &String::new();
        let s0 = s.borrow_writable_internal();

        let s = &mut String::new();
        let s0 = s.borrow_writable_internal();

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
            writable::{Writable, WritableStr},
            write::{Flush, Write, WriteInfallible},
        };

        struct Test(bool);

        impl Writable for Test {
            fn write_to<W>(&self, w: &mut W) -> Result<(), W::Error>
            where
                W: Write + ?Sized,
            {
                w.write_str(self.0.str())
            }
        }

        impl WritableDebug for Test {
            fn write_to_debug<W>(&self, w: &mut W) -> Result<(), W::Error>
            where
                W: Write + ?Sized,
            {
                w.write_str(if self.0 { "Test(true)" } else { "Test(false)" })
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
        Test(true).write_to_debug(&mut s).into_ok();
        assert_eq!(s, "Test(true)");

        let mut s = String::new();
        Test(false).write_to_debug(&mut s).into_ok();
        assert_eq!(s, "Test(false)");

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

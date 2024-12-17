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

pub trait WritablePrecision {
    fn write_to_precision<W, const PRECISION: u8>(&self, w: &mut W) -> Result<(), W::Error>
    where
        W: Write + ?Sized;

    #[inline]
    fn len_hint(&self) -> usize {
        0
    }
}

pub trait BorrowWritable {
    type Target: ?Sized + Writable;
    fn borrow_writable(&self) -> &Self::Target;
}
impl<T> BorrowWritable for T
where
    T: Writable + ?Sized,
{
    type Target = Self;
    fn borrow_writable(&self) -> &Self::Target {
        self
    }
}

pub trait WritableInternal: Writable {
    fn write_to_internal<W, const PRECISION: u8, const ALT: bool>(
        &self,
        w: &mut W,
    ) -> Result<(), W::Error>
    where
        W: Write + ?Sized;

    fn len_hint_internal(&self) -> usize;

    fn borrow_writable_internal(&self) -> &Self;
}
impl<T> WritableInternal for T
where
    T: Writable + ?Sized,
{
    fn write_to_internal<W, const PRECISION: u8, const ALT: bool>(
        &self,
        w: &mut W,
    ) -> Result<(), W::Error>
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
    fn write_to_internal<W, const PRECISION: u8, const ALT: bool>(
        &self,
        w: &mut W,
    ) -> Result<(), W::Error>
    where
        W: Write + ?Sized;

    fn len_hint_internal(&self) -> usize;

    fn borrow_writable_internal(&self) -> &Self;
}
impl<T> WritableDebugInternal for T
where
    T: WritableDebug + ?Sized,
{
    fn write_to_internal<W, const PRECISION: u8, const ALT: bool>(
        &self,
        w: &mut W,
    ) -> Result<(), W::Error>
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
    fn write_to_internal<W, const PRECISION: u8, const ALT: bool>(
        &self,
        w: &mut W,
    ) -> Result<(), W::Error>
    where
        W: Write + ?Sized;

    fn len_hint_internal(&self) -> usize;

    fn borrow_writable_internal(&self) -> &Self;
}
impl<T> WritableBinaryInternal for T
where
    T: WritableBinary + ?Sized,
{
    fn write_to_internal<W, const PRECISION: u8, const ALT: bool>(
        &self,
        w: &mut W,
    ) -> Result<(), W::Error>
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
    fn write_to_internal<W, const PRECISION: u8, const ALT: bool>(
        &self,
        w: &mut W,
    ) -> Result<(), W::Error>
    where
        W: Write + ?Sized;

    fn len_hint_internal(&self) -> usize;

    fn borrow_writable_internal(&self) -> &Self;
}
impl<T> WritableHexadecimalInternal for T
where
    T: WritableHexadecimal + ?Sized,
{
    fn write_to_internal<W, const PRECISION: u8, const ALT: bool>(
        &self,
        w: &mut W,
    ) -> Result<(), W::Error>
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

pub trait WritablePrecisionInternal {
    type Target: ?Sized;

    fn write_to_internal<W, const PRECISION: u8, const ALT: bool>(
        &self,
        w: &mut W,
    ) -> Result<(), W::Error>
    where
        W: Write + ?Sized;

    fn len_hint_internal(&self) -> usize;

    fn borrow_writable_internal(&self) -> &Self;
}
impl<T> WritablePrecisionInternal for T
where
    T: WritablePrecision,
{
    type Target = Self;

    fn write_to_internal<W, const PRECISION: u8, const ALT: bool>(
        &self,
        w: &mut W,
    ) -> Result<(), W::Error>
    where
        W: Write + ?Sized,
    {
        self.write_to_precision::<W, PRECISION>(w)
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

impl WritableDebug for str {
    #[inline]
    fn write_to_debug<W>(&self, w: &mut W) -> Result<(), W::Error>
    where
        W: Write + ?Sized,
    {
        w.write_str(self)
    }

    #[inline]
    fn len_hint(&self) -> usize {
        self.len()
    }
}

pub trait WritableConstStr {
    const CONST_STR: &'static str;
}

pub trait WritableStaticStr {
    fn static_str(&self) -> &'static str;
}

impl<T> WritableStaticStr for T
where
    T: WritableConstStr + ?Sized,
{
    #[inline]
    fn static_str(&self) -> &'static str {
        Self::CONST_STR
    }
}

pub trait WritableStr {
    fn str(&self) -> &str;
}

impl<T> WritableStr for T
where
    T: WritableStaticStr + ?Sized,
{
    #[inline]
    fn str(&self) -> &str {
        self.static_str()
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

impl WritableStaticStr for bool {
    fn static_str(&self) -> &'static str {
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
macro_rules! impl_writable_for_display {
	{ $($name:ty ),* $(,)? } => {
		$(
			impl Writable for $name {
				#[inline]
				fn write_to<W>(&self, w: &mut W) -> Result<(), W::Error>
					where
						W: $crate::write::Write + ?Sized {
					w.write_stdfmtdisplay(self)
				}
			}
		)*
	};
}

macro_rules! impl_writable_int_for_display {
	{ $($name:ty ),* $(,)? } => {
		$(
			impl $crate::writable::Writable for $name {
				#[inline]
				fn write_to<W>(&self, w: &mut W) -> Result<(), W::Error>
					where
						W: $crate::write::Write + ?Sized {
					w.write_stdfmtdisplay(self)
				}
			}
			impl $crate::writable::WritableDebug for $name {
				#[inline]
				fn write_to_debug<W>(&self, w: &mut W) -> Result<(), W::Error>
					where
						W: $crate::write::Write + ?Sized {
					w.write_stdfmtdebug(self)
				}
			}
			impl $crate::writable::WritableBinary for $name {
				#[inline]
				fn write_to_binary<W>(&self, w: &mut W) -> Result<(), W::Error>
					where
						W: $crate::write::Write + ?Sized {
					w.write_stdfmtbinary(self)
				}
			}
			impl $crate::writable::WritableHexadecimal for $name {
				#[inline]
				fn write_to_hexadecimal<W>(&self, w: &mut W) -> Result<(), W::Error>
					where
						W: $crate::write::Write + ?Sized {
					w.write_stdfmthexadecimal(self)
				}
			}
		)*
	};
}
macro_rules! impl_writable_float_for_display {
	{ $($name:ty ),* $(,)? } => {
		$(
			impl $crate::writable::Writable for $name {
				#[inline]
				fn write_to<W>(&self, w: &mut W) -> Result<(), W::Error>
					where
						W: $crate::write::Write + ?Sized {
					w.write_stdfmtdisplay(self)
				}
			}
			impl $crate::writable::WritableDebug for $name {
				#[inline]
				fn write_to_debug<W>(&self, w: &mut W) -> Result<(), W::Error>
					where
						W: $crate::write::Write + ?Sized {
					w.write_stdfmtdebug(self)
				}
			}
			impl $crate::writable::WritablePrecision for $name {
				#[inline]
				fn write_to_precision<W, const PRECISION: u8>(&self, w: &mut W) -> Result<(), W::Error>
					where
						W: $crate::write::Write + ?Sized {
					w.write_stdfmtprecision(self, Some(PRECISION as usize))
				}
			}
		)*
	};
}

impl_writable_int_for_display!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);
impl_writable_float_for_display!(f32, f64);

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

    use crate::writable::{Writable, WritableDebug};

    use super::ToString;

    #[test]
    fn borrow() {
        use super::WritableInternal;

        let i = 32;
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

        struct OwnedWritable<'a, T>(&'a T)
        where
            T: Writable + ?Sized;

        // impl<'a, T> Writable for OwnedWritable<'a, T>
        // where
        //     T: Writable + ?Sized,
        // {
        //     fn write_to<W>(&self, w: &mut W) -> Result<(), W::Error>
        //     where
        //         W: crate::write::Write + ?Sized,
        //     {
        //         w.write(self.0)
        //     }
        // }
    }

    #[test]
    fn write() {
        use crate::{
            writable::{Writable, WritableStr},
            write::{Flush, Write, WriteFlush, WriteInfallible},
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

        fn takes_write_flush<W, E>(w: &W)
        where
            W: WriteFlush<_Error = E>,
        {
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

        takes_write_flush(&stdout);
    }
}

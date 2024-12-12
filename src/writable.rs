pub trait Writable {
    fn write_to<W>(&self, w: &mut W) -> Result<(), W::Error>
    where
        W: Write + ?Sized;
}

pub trait WritableDebug {
    fn write_to_debug<W>(&self, w: &mut W) -> Result<(), W::Error>
    where
        W: Write + ?Sized;
}

// impl Writable for str {
//     fn write_to<W>(&self, w: &mut W) -> Result<(), W::Error>
//     where
//         W: Write + ?Sized,
//     {
//         w.write_str(self)
//     }
// }

impl WritableDebug for str {
    fn write_to_debug<W>(&self, w: &mut W) -> Result<(), W::Error>
    where
        W: Write + ?Sized,
    {
        w.write_str(self)
    }
}

impl WritableDebug for &str {
    fn write_to_debug<W>(&self, w: &mut W) -> Result<(), W::Error>
    where
        W: Write + ?Sized,
    {
        w.write_str(self)
    }
}

pub trait WritableConstStr {
    const CONST_STR: &'static str;
}

pub trait WritableStaticStr {
    fn static_str(&self) -> &'static str;
}

pub trait WritableStr {
    fn str(&self) -> &str;
}

impl WritableStr for str {
    #[inline]
    fn str(&self) -> &str {
        self
    }
}

// impl WritableStr for &str {
//     #[inline]
//     fn str(&self) -> &str {
//         self
//     }
// }
//
// impl WritableStr for String {
//     #[inline]
//     fn str(&self) -> &str {
//         self
//     }
// }

impl<T> WritableStaticStr for T
where
    T: WritableConstStr + ?Sized,
{
    #[inline]
    fn static_str(&self) -> &'static str {
        Self::CONST_STR
    }
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
        let mut s = String::new();
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

// impl<T, U> WritableStr for WithWritableStr<T, U>
// where
//     U: WritableStr,
// {
//     fn str(&self) -> &str {
//         self.writable.str()
//     }
// }
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

// impl<T0, T1, U> Deref for WithWritableStr<T0, U>
// where
//     T0: Deref<Target = T1>,
//     T1: ?Sized,
// {
//     type Target = T1;
//
//     #[inline]
//     #[expect(clippy::explicit_deref_methods)]
//     fn deref(&self) -> &Self::Target {
//         self.value.deref()
//     }
// }
impl<T, U> Deref for WithWritableStr<T, U> {
    type Target = Self;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self
    }
}

pub struct FmtDisplayWritable<'d, D>(pub &'d D)
where
    D: core::fmt::Display + ?Sized;

impl<D> Writable for FmtDisplayWritable<'_, D>
where
    D: core::fmt::Display + ?Sized,
{
    #[inline]
    fn write_to<W>(&self, w: &mut W) -> Result<(), W::Error>
    where
        W: Write + ?Sized,
    {
        w.write_fmtdisplay(self.0)
    }
}

pub trait FmtDisplayAsWriteTo: core::fmt::Display {
    fn fmt_display_as_write_to(&self) -> FmtDisplayWritable<Self>;
}

impl<D> FmtDisplayAsWriteTo for D
where
    D: core::fmt::Display,
{
    #[inline]
    fn fmt_display_as_write_to(&self) -> FmtDisplayWritable<Self> {
        FmtDisplayWritable(self)
    }
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
					w.write_fmtdisplay(self)
				}
			}
		)*
	};
}
use core::ops::Deref;

impl_writable_for_display!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);

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
macro_rules! impl_display_for_writable {
	{ $($name:ty),* $(,)? } => {
		$(
			impl ::core::fmt::Display for $name {
				#[inline]
				fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
					::fmt2::writable::Writable::write_to(self, f)
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

#[macro_export]
macro_rules! impl_write_flush_for_io_write {
	($($ty:ty),* $(,)?) => {
		$(
			impl $crate::write::Write for $ty {
				type Error = ::std::io::Error;

				#[inline]
				fn write_str(&mut self, s: &str) -> ::core::result::Result<(), Self::Error> {
					::std::io::Write::write_all(self, s.as_bytes())
				}
			}

			impl $crate::write::Flush for $ty {
				type Error = ::std::io::Error;

				#[inline]
				fn flush(&mut self) -> ::core::result::Result<(), Self::Error> {
					::std::io::Write::flush(self)
				}
			}
		)*
	};
}

use crate::write::Write;

impl_write_flush_for_io_write!(
    std::io::Stdout,
    std::io::StdoutLock<'_>,
    std::io::Stderr,
    std::io::StderrLock<'_>
);

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
fn test() {
    // {
    // 		#[derive(Clone, Copy)]
    // 		pub struct A<T, U = &'static str> {
    // 			pub value: T,
    // 			pub writable: U,
    // 		}
    //
    // 		impl<T, U> A<T, U> {
    // 			pub const fn new(value: T, displayable: U) -> Self {
    // 				Self {
    // 					value,
    // 					writable: displayable,
    // 				}
    // 			}
    // 		}
    //
    // 		impl<T0, T1, U> AsRef<T1> for A<T0, U>
    // 		where
    // 			T0: AsRef<T1>,
    // 			T1: ?Sized,
    // 		{
    // 			#[inline]
    // 			fn as_ref(&self) -> &T1 {
    // 				self.value.as_ref()
    // 			}
    // 		}
    //
    // 		impl<T0, T1, U> Deref for A<T0, U>
    // 		where
    // 			T0: Deref<Target = T1>,
    // 			T1: ?Sized,
    // 		{
    // 			type Target = T1;
    //
    // 			#[inline]
    // 			#[expect(clippy::explicit_deref_methods)]
    // 			fn deref(&self) -> &Self::Target {
    // 				self.value.deref()
    // 			}
    // 		}
    //
    // 		impl<T, U> WritableStr for A<T, U>
    // 		where
    // 			U: WritableStr,
    // 		{
    // 			fn str(&self) -> &str {
    // 				self.writable.str()
    // 			}
    // 		}
    //
    // 		fn deref<'a, T>(a1: &'a T) -> &'a i32
    // 		where
    // 			T: Deref<Target = i32>,
    // 		{
    // 			Deref::deref(a1)
    // 		}
    // 		fn deref2<T>(a1: T) -> i32
    // 		where
    // 			T: Deref<Target = i32>,
    // 		{
    // 			let a = Deref::deref(&a1);
    // 			let b: i32 = *a;
    // 			b
    // 		}
    //
    // 		// fn f<T>(t: T) -> &i32
    // 		// where
    // 		// 	T: Deref<Target = i32>,
    // 		// {
    // 		// 	let t = t.deref();
    // 		// }
    //
    // 		type X<'a> = <&'a A<&'a A<&'a i32>> as Deref>::Target;
    //
    // 		let x: X;
    //
    // 		let a0 = A::new(&1_i32, "abc");
    // 		let a1 = &A::new(&a0, "abc");
    //
    // 		// let a2 = deref(a1);
    // 		let a2: &i32 = Deref::deref(a1);
    // 		let a2 = Deref::deref(Deref::deref(a1));
    // 		let a2 = Deref::deref(a1);
    // 		// let a2 = deref(a0);
    // 		let c = deref2(a0);
    // 		let b = Deref::deref(&a0);
    // 	}

    use crate::write::{Flush, Write, WriteFlush, WriteInfallible};

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

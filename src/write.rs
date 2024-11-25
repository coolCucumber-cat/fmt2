fn test2() {}

#[expect(clippy::module_name_repetitions)]
pub trait WriteInfallible {
    fn write_str_infallible(&mut self, s: &str);

    fn write_infallible<WT>(&mut self, wt: &WT)
    where
        WT: WriteTo + ?Sized,
    {
        wt.write_to(self).into_ok();
    }
}

pub trait Write {
    type Error;

    fn write_str(&mut self, s: &str) -> Result<(), Self::Error>;

    fn writeln_str(&mut self, s: &str) -> Result<(), Self::Error> {
        self.write_str(s)?;
        self.write_str("\n")
    }

    fn write_char(&mut self, c: char) -> Result<(), Self::Error> {
        self.write_str(c.encode_utf8(&mut [0; 4]))
    }

    fn write<WT>(&mut self, wt: &WT) -> Result<(), Self::Error>
    where
        WT: WriteTo + ?Sized,
    {
        wt.write_to(self)
    }

    fn writeln<WT>(&mut self, wt: &WT) -> Result<(), Self::Error>
    where
        WT: WriteTo + ?Sized,
    {
        wt.write_to(self)?;
        self.write("\n")
    }

    fn write_map<WT>(&mut self, wt: &WT) -> Result<&mut Self, Self::Error>
    where
        WT: WriteTo + ?Sized,
    {
        wt.write_to(self).map(|()| self)
    }

    fn fmtwrite_adapter(
        &mut self,
        f: impl FnOnce(&mut dyn core::fmt::Write) -> core::fmt::Result,
    ) -> Result<(), Self::Error> {
        struct Adapter<'w, W>
        where
            W: Write + ?Sized,
        {
            write: &'w mut W,
            result: Result<(), W::Error>,
        }

        impl<W> core::fmt::Write for Adapter<'_, W>
        where
            W: Write + ?Sized,
        {
            fn write_str(&mut self, s: &str) -> core::fmt::Result {
                match self.write.write_str(s) {
                    Ok(()) => Ok(()),
                    Err(e) => {
                        self.result = Err(e);
                        Err(core::fmt::Error)
                    }
                }
            }
        }

        let mut write = Adapter {
            write: self,
            result: Ok(()),
        };
        if f(&mut write).is_ok() {
            Ok(())
        } else {
            // it's possible for Display to error on its own, but it should be write doing the err
            debug_assert!(write.result.is_err());
            write.result
        }
    }

    fn write_fmtdisplay<D>(&mut self, d: &D) -> Result<(), Self::Error>
    where
        D: core::fmt::Display + ?Sized,
    {
        self.fmtwrite_adapter(|w| d.fmt(&mut core::fmt::Formatter::new(w)))
    }

    fn write_fmtdebug<D>(&mut self, d: &D) -> Result<(), Self::Error>
    where
        D: core::fmt::Debug + ?Sized,
    {
        self.fmtwrite_adapter(|w| d.fmt(&mut core::fmt::Formatter::new(w)))
    }

    fn write_fmtargs(&mut self, args: core::fmt::Arguments<'_>) -> Result<(), Self::Error> {
        self.fmtwrite_adapter(|w| core::fmt::write(w, args))
    }
}

pub trait Flush {
    type Error;

    fn flush(&mut self) -> Result<(), Self::Error>;
}

pub trait WriteFlush: Write<Error = Self::_Error> + Flush<Error = Self::_Error> {
    type _Error;
}

impl<WF, E> WriteFlush for WF
where
    WF: Write<Error = E> + Flush<Error = E>,
{
    type _Error = E;
}

impl<W> Write for W
where
    W: WriteInfallible + ?Sized,
{
    type Error = !;

    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        self.write_str_infallible(s);
        Ok(())
    }
}

impl WriteInfallible for String {
    fn write_str_infallible(&mut self, s: &str) {
        self.push_str(s);
    }
}

// impl<W> WriteInfallible for W
// where
// 	W: Write<Error = !> + ?Sized,
// {
// 	fn write_str_infallible(&mut self, s: &str) {
// 		self.write_str(s).into_ok();
// 	}
// }

// impl Write for String {
// 	type Error = !;
//
// 	fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
// 		self.push_str(s);
// 		Ok(())
// 	}
// }

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
pub(crate) use impl_write_flush_for_io_write;

impl_write_flush_for_io_write!(
    std::io::Stdout,
    std::io::StdoutLock<'_>,
    std::io::Stderr,
    std::io::StderrLock<'_>
);

#[expect(clippy::module_name_repetitions)]
pub trait WriteTo {
    fn write_to<W>(&self, w: &mut W) -> Result<(), W::Error>
    where
        W: Write + ?Sized;
}

#[expect(clippy::module_name_repetitions)]
pub trait WriteToDebug {
    fn write_to_debug<W>(&self, w: &mut W) -> Result<(), W::Error>
    where
        W: Write + ?Sized;
}

pub struct FmtDisplayWriteTo<'d, D>(&'d D)
where
    D: core::fmt::Display + ?Sized;

impl<D> WriteTo for FmtDisplayWriteTo<'_, D>
where
    D: core::fmt::Display + ?Sized,
{
    fn write_to<W>(&self, w: &mut W) -> Result<(), W::Error>
    where
        W: Write + ?Sized,
    {
        w.write_fmtdisplay(self.0)
    }
}

pub trait FmtDisplayAsWriteTo: core::fmt::Display {
    fn fmt_display_as_write_to(&self) -> FmtDisplayWriteTo<Self>;
}

impl<D> FmtDisplayAsWriteTo for D
where
    D: core::fmt::Display,
{
    fn fmt_display_as_write_to(&self) -> FmtDisplayWriteTo<Self> {
        FmtDisplayWriteTo(self)
    }
}

impl WriteTo for str {
    fn write_to<W>(&self, w: &mut W) -> Result<(), W::Error>
    where
        W: Write + ?Sized,
    {
        w.write_str(self)
    }
}

// impl WriteTo for &str {
// 	fn write_to<W>(&self, w: &mut W) -> Result<(), W::Error>
// 	where
// 		W: Write + ?Sized,
// 	{
// 		w.write_str(self)
// 	}
// }

impl WriteToDebug for str {
    fn write_to_debug<W>(&self, w: &mut W) -> Result<(), W::Error>
    where
        W: Write + ?Sized,
    {
        w.write_str(self)
    }
}

// impl WriteToDebug for &str {
// 	fn write_to_debug<W>(&self, w: &mut W) -> Result<(), W::Error>
// 	where
// 		W: Write + ?Sized,
// 	{
// 		w.write_str(self)
// 	}
// }

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
    fn x<T>(t: &T) {
        let f = |w: &T| {};

        f(t);
    }

    struct Test(bool);

    impl WriteTo for Test {
        fn write_to<W>(&self, w: &mut W) -> Result<(), W::Error>
        where
            W: Write + ?Sized,
        {
            w.write_str(if self.0 { "true" } else { "false" })
        }
    }

    impl WriteToDebug for Test {
        fn write_to_debug<W>(&self, w: &mut W) -> Result<(), W::Error>
        where
            W: Write + ?Sized,
        {
            w.write_str(if self.0 { "Test(true)" } else { "Test(false)" })
        }
    }

    fn takes_write_flush<W, E>(w: &W)
    where
        W: WriteFlush<_Error = E>,
    {
    }

    let mut s = String::new();
    s.write_infallible(&Test(true));
    assert_eq!(s, "true");

    let mut s = String::new();
    s.write_infallible(&Test(false));
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

    takes_write_flush(&stdout);
}

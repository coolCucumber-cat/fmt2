use crate::writable::{Writable, WritableDebug};

pub trait Write {
    type Error;

    fn write_str(&mut self, s: &str) -> Result<(), Self::Error>;

    #[inline]
    fn writeln_str(&mut self, s: &str) -> Result<(), Self::Error> {
        self.write_str(s)?;
        self.write_newline()
    }

    #[inline]
    fn write<WT>(&mut self, wt: &WT) -> Result<(), Self::Error>
    where
        WT: Writable + ?Sized,
    {
        self.write_without_flush_hint_(wt)?;
        self.flush_hint();
        Ok(())
    }

    #[inline]
    fn write_without_flush_hint_<WT>(&mut self, wt: &WT) -> Result<(), Self::Error>
    where
        WT: Writable + ?Sized,
    {
        wt.write_to(self)
    }

    #[inline]
    fn writeln<WT>(&mut self, wt: &WT) -> Result<(), Self::Error>
    where
        WT: Writable + ?Sized,
    {
        self.write_without_flush_hint_(wt)?;
        self.write_newline()?;
        self.flush_hint();
        Ok(())
    }

    #[inline]
    fn write_debug<WT>(&mut self, wt: &WT) -> Result<(), Self::Error>
    where
        WT: WritableDebug + ?Sized,
    {
        self.write_debug_without_flush_hint_(wt)?;
        self.flush_hint();
        Ok(())
    }

    #[inline]
    fn write_debug_without_flush_hint_<WT>(&mut self, wt: &WT) -> Result<(), Self::Error>
    where
        WT: WritableDebug + ?Sized,
    {
        wt.write_to_debug(self)
    }

    #[inline]
    fn write_char(&mut self, c: char) -> Result<(), Self::Error> {
        self.write_str(c.encode_utf8(&mut [0; 4]))
    }

    #[inline]
    fn write_newline(&mut self) -> Result<(), Self::Error> {
        self.write_str("\n")
    }

    #[inline]
    fn write_stdfmtdisplay<D>(&mut self, d: &D) -> Result<(), Self::Error>
    where
        D: core::fmt::Display + ?Sized,
    {
        stdfmtwrite_adapter(self, |w| {
            d.fmt(&mut core::fmt::Formatter::new(
                w,
                core::fmt::FormattingOptions::new(),
            ))
        })
    }

    #[inline]
    fn write_stdfmtdebug<D>(&mut self, d: &D) -> Result<(), Self::Error>
    where
        D: core::fmt::Debug + ?Sized,
    {
        stdfmtwrite_adapter(self, |w| {
            d.fmt(&mut core::fmt::Formatter::new(
                w,
                core::fmt::FormattingOptions::new(),
            ))
        })
    }

    #[inline]
    fn write_stdfmtprecision<D>(
        &mut self,
        d: &D,
        precision: Option<usize>,
    ) -> Result<(), Self::Error>
    where
        D: core::fmt::Display + ?Sized,
    {
        stdfmtwrite_adapter(self, |w| {
            let mut options = core::fmt::FormattingOptions::new();
            options.precision(precision);
            d.fmt(&mut core::fmt::Formatter::new(w, options))
        })
    }

    #[inline]
    fn write_stdfmtbinary<D>(&mut self, d: &D) -> Result<(), Self::Error>
    where
        D: core::fmt::Binary + ?Sized,
    {
        stdfmtwrite_adapter(self, |w| {
            d.fmt(&mut core::fmt::Formatter::new(
                w,
                core::fmt::FormattingOptions::new(),
            ))
        })
    }

    #[inline]
    fn write_stdfmthexadecimal<D>(&mut self, d: &D) -> Result<(), Self::Error>
    where
        D: core::fmt::LowerHex + ?Sized,
    {
        stdfmtwrite_adapter(self, |w| {
            d.fmt(&mut core::fmt::Formatter::new(
                w,
                core::fmt::FormattingOptions::new(),
            ))
        })
    }

    #[inline]
    fn write_stdfmtargs(&mut self, args: core::fmt::Arguments<'_>) -> Result<(), Self::Error> {
        if let Some(s) = args.as_str() {
            self.write_str(s)
        } else {
            stdfmtwrite_adapter(self, |w| core::fmt::write(w, args))
        }
    }

    #[inline]
    fn flush_hint(&mut self) {}
}

impl<W> Write for W
where
    W: WriteInfallible + ?Sized,
{
    type Error = !;

    #[inline]
    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        self.write_str_infallible(s);
        Ok(())
    }
}

impl Write for core::fmt::Formatter<'_> {
    type Error = core::fmt::Error;

    #[inline]
    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        core::fmt::Formatter::write_str(self, s)
    }

    #[inline]
    fn write_stdfmtdisplay<D>(&mut self, d: &D) -> Result<(), Self::Error>
    where
        D: core::fmt::Display + ?Sized,
    {
        d.fmt(self)
    }

    #[inline]
    fn write_stdfmtdebug<D>(&mut self, d: &D) -> Result<(), Self::Error>
    where
        D: core::fmt::Debug + ?Sized,
    {
        d.fmt(self)
    }

    #[inline]
    fn write_stdfmtargs(&mut self, args: core::fmt::Arguments<'_>) -> Result<(), Self::Error> {
        self.write_fmt(args)
    }
}

// impl Write for String {
// 	type Error = !;
//
// 	fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
// 		self.push_str(s);
// 		Ok(())
// 	}
// }

#[inline]
fn stdfmtwrite_adapter<W0>(
    write: &mut W0,
    f: impl FnOnce(&mut dyn core::fmt::Write) -> core::fmt::Result,
) -> Result<(), W0::Error>
where
    W0: Write + ?Sized,
{
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
        #[inline]
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
        write,
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

#[expect(clippy::module_name_repetitions)]
pub trait WriteInfallible {
    fn write_str_infallible(&mut self, s: &str);

    // fn write_infallible<WT>(&mut self, wt: &WT)
    // where
    // 	WT: Writable + ?Sized,
    // {
    // 	wt.write_to(self).into_ok();
    // }
}

impl WriteInfallible for String {
    #[inline]
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

                #[inline]
                fn writeln<WT>(&mut self, wt: &WT) -> Result<(), Self::Error>
                where
                    WT: Writable + ?Sized,
                {
                    self.write_without_flush_hint_(wt)?;
                    self.write_newline()
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

impl_write_flush_for_io_write!(
    std::io::Stdout,
    std::io::StdoutLock<'_>,
    std::io::Stderr,
    std::io::StderrLock<'_>
);

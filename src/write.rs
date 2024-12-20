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
    fn write_std_display<D>(&mut self, d: &D) -> Result<(), Self::Error>
    where
        D: core::fmt::Display + ?Sized,
    {
        self.std_formatter_adapter(|f| d.fmt(f))
    }

    #[inline]
    fn write_std_debug<D>(&mut self, d: &D) -> Result<(), Self::Error>
    where
        D: core::fmt::Debug + ?Sized,
    {
        self.std_formatter_adapter(|f| d.fmt(f))
    }

    #[inline]
    fn write_std_binary<D>(&mut self, d: &D) -> Result<(), Self::Error>
    where
        D: core::fmt::Binary + ?Sized,
    {
        self.std_formatter_adapter(|f| d.fmt(f))
    }

    #[inline]
    fn write_std_octal<D>(&mut self, d: &D) -> Result<(), Self::Error>
    where
        D: core::fmt::Octal + ?Sized,
    {
        self.std_formatter_adapter(|f| d.fmt(f))
    }

    #[inline]
    fn write_std_hex<D>(&mut self, d: &D) -> Result<(), Self::Error>
    where
        D: core::fmt::UpperHex + ?Sized,
    {
        self.std_formatter_adapter(|f| d.fmt(f))
    }

    #[inline]
    fn write_std_precision<D>(&mut self, d: &D, precision: Option<usize>) -> Result<(), Self::Error>
    where
        D: core::fmt::Display + ?Sized,
    {
        let mut options = core::fmt::FormattingOptions::new();
        options.precision(precision);
        self.std_formatter_with_options_adapter(options, |f| d.fmt(f))
    }

    #[inline]
    fn write_std_args(&mut self, args: core::fmt::Arguments<'_>) -> Result<(), Self::Error> {
        if let Some(s) = args.as_str() {
            self.write_str(s)
        } else {
            self.std_write_adapter(|w| core::fmt::write(w, args))
        }
    }

    #[inline]
    fn flush_hint(&mut self) {}

    #[inline]
    fn flush_hint_after_newline(&mut self) {
        self.flush_hint();
    }

    #[inline]
    fn std_write_adapter(
        &mut self,
        f: impl FnOnce(&mut dyn core::fmt::Write) -> core::fmt::Result,
    ) -> Result<(), Self::Error> {
        struct Adapter<'w, W>
        where
            W: Write + ?Sized,
        {
            writer: &'w mut W,
            result: Result<(), W::Error>,
        }

        impl<W> core::fmt::Write for Adapter<'_, W>
        where
            W: Write + ?Sized,
        {
            #[inline]
            fn write_str(&mut self, s: &str) -> core::fmt::Result {
                match self.writer.write_str(s) {
                    Ok(()) => Ok(()),
                    Err(e) => {
                        self.result = Err(e);
                        Err(core::fmt::Error)
                    }
                }
            }
        }

        let mut write = Adapter {
            writer: self,
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

    #[inline]
    fn std_formatter_with_options_adapter(
        &mut self,
        options: core::fmt::FormattingOptions,
        f: impl FnOnce(&mut core::fmt::Formatter) -> core::fmt::Result,
    ) -> Result<(), Self::Error> {
        self.std_write_adapter(|w| {
            let formatter = &mut core::fmt::Formatter::new(w, options);
            f(formatter)
        })
    }

    #[inline]
    fn std_formatter_adapter(
        &mut self,
        f: impl FnOnce(&mut core::fmt::Formatter) -> core::fmt::Result,
    ) -> Result<(), Self::Error> {
        self.std_write_adapter(|w| {
            let formatter = &mut core::fmt::Formatter::new(w, core::fmt::FormattingOptions::new());
            f(formatter)
        })
    }
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
    fn write_std_args(&mut self, args: core::fmt::Arguments<'_>) -> Result<(), Self::Error> {
        self.write_fmt(args)
    }

    #[inline]
    fn std_write_adapter(
        &mut self,
        f: impl FnOnce(&mut dyn core::fmt::Write) -> core::fmt::Result,
    ) -> Result<(), Self::Error> {
        f(self)
    }

    #[inline]
    fn std_formatter_with_options_adapter(
        &mut self,
        options: core::fmt::FormattingOptions,
        f: impl FnOnce(&mut core::fmt::Formatter) -> core::fmt::Result,
    ) -> Result<(), Self::Error> {
        let formatter = &mut self.with_options(options);
        f(formatter)
    }

    #[inline]
    fn std_formatter_adapter(
        &mut self,
        f: impl FnOnce(&mut core::fmt::Formatter) -> core::fmt::Result,
    ) -> Result<(), Self::Error> {
        f(self)
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

pub trait WriteInternal: Write {
    fn borrow_write_internal(&mut self) -> &mut Self;
}

impl<T> WriteInternal for T
where
    T: Write + ?Sized,
{
    #[inline]
    fn borrow_write_internal(&mut self) -> &mut Self {
        self
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
                fn flush_hint(&mut self) {
                    let _ = $crate::write::Flush::flush(self);
                }

                #[inline]
                fn flush_hint_after_newline(&mut self) {}
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

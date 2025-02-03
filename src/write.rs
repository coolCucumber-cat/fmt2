use crate::write_to::WriteTo;

pub trait Write {
    type Error;

    const IS_LINE_BUFFERED: bool;

    fn write_str(&mut self, s: &str) -> Result<(), Self::Error>;

    #[inline]
    fn writeln_str(&mut self, s: &str) -> Result<(), Self::Error> {
        self.write_str(s)?;
        // self.write_newline()
        self.write_str("\n")
    }

    #[inline]
    fn write_advanced<WT, const FLUSH: bool, const NEWLINE: bool>(
        &mut self,
        wt: &WT,
    ) -> Result<(), Self::Error>
    where
        WT: WriteTo + ?Sized,
    {
        wt.write_to(self)?;
        if NEWLINE {
            // self.write_newline()?;
            self.write_str("\n")?;
        }
        // if a newline is the last thing written to a life buffered writer, it's already flushed.
        // only flush if we want to flush and if it isnt already confirmed to be flushed
        // we would use `flsuh_advanced` if we could, but it is not allowed, due to the generic
        if FLUSH && !(Self::IS_LINE_BUFFERED && (NEWLINE || WT::ENDS_IN_NEWLINE)) {
            self.flush_hint();
        }
        Ok(())
    }

    #[inline]
    fn write<WT>(&mut self, wt: &WT) -> Result<(), Self::Error>
    where
        WT: WriteTo + ?Sized,
    {
        self.write_advanced::<WT, true, false>(wt)
    }

    #[inline]
    fn writeln<WT>(&mut self, wt: &WT) -> Result<(), Self::Error>
    where
        WT: WriteTo + ?Sized,
    {
        self.write_advanced::<WT, true, true>(wt)
    }

    #[inline]
    fn write_char(&mut self, c: char) -> Result<(), Self::Error> {
        self.write_str(c.encode_utf8(&mut [0; 4]))
    }

    // #[inline]
    // fn write_newline(&mut self) -> Result<(), Self::Error> {
    //     self.write_str("\n")
    // }

    #[inline]
    fn flush_hint(&mut self) {}

    #[inline]
    fn flush_hint_advanced<const FLUSH: bool, const BUFFER_ENDS_IN_NEWLINE: bool>(&mut self) {
        // if a newline is the last thing written to a life buffered writer, it's already flushed.
        // only flush if we want to flush and if it isnt already confirmed to be flushed
        if FLUSH && !(BUFFER_ENDS_IN_NEWLINE && Self::IS_LINE_BUFFERED) {
            self.flush_hint();
        }
    }

    #[inline]
    fn write_std_display<D>(&mut self, d: &D) -> Result<(), Self::Error>
    where
        D: core::fmt::Display + ?Sized,
    {
        #[cfg(feature = "fmt_internals")]
        {
            self.std_formatter_adapter(|f| d.fmt(f))
        }
        #[cfg(not(feature = "fmt_internals"))]
        {
            self.write_std_args(core::format_args!("{d}"))
        }
    }

    #[inline]
    fn write_std_debug<D>(&mut self, d: &D) -> Result<(), Self::Error>
    where
        D: core::fmt::Debug + ?Sized,
    {
        #[cfg(feature = "fmt_internals")]
        {
            self.std_formatter_adapter(|f| d.fmt(f))
        }
        #[cfg(not(feature = "fmt_internals"))]
        {
            self.write_std_args(core::format_args!("{d:?}"))
        }
    }

    #[inline]
    fn write_std_binary<D>(&mut self, d: &D) -> Result<(), Self::Error>
    where
        D: core::fmt::Binary + ?Sized,
    {
        #[cfg(feature = "fmt_internals")]
        {
            self.std_formatter_adapter(|f| d.fmt(f))
        }
        #[cfg(not(feature = "fmt_internals"))]
        {
            self.write_std_args(core::format_args!("{d:b}"))
        }
    }

    #[inline]
    fn write_std_octal<D>(&mut self, d: &D) -> Result<(), Self::Error>
    where
        D: core::fmt::Octal + ?Sized,
    {
        #[cfg(feature = "fmt_internals")]
        {
            let unusawdawdjawdjawd = 1;
            self.std_formatter_adapter(|f| d.fmt(f))
        }
        #[cfg(not(feature = "fmt_internals"))]
        {
            self.write_std_args(core::format_args!("{d:o}"))
        }
    }

    #[inline]
    fn write_std_upper_hex<D>(&mut self, d: &D) -> Result<(), Self::Error>
    where
        D: core::fmt::UpperHex + ?Sized,
    {
        #[cfg(feature = "fmt_internals")]
        {
            self.std_formatter_adapter(|f| d.fmt(f))
        }
        #[cfg(not(feature = "fmt_internals"))]
        {
            self.write_std_args(core::format_args!("{d:X}"))
        }
    }

    #[cfg(feature = "fmt_internals")]
    #[inline]
    fn write_std_precision<D, const PRECISION: u8>(&mut self, d: &D) -> Result<(), Self::Error>
    where
        D: core::fmt::Display + ?Sized,
    {
        #[cfg(not(feature = "formatting_options"))]
        {
            self.std_formatter_adapter(|f| d.fmt(f))
        }
        #[cfg(feature = "formatting_options")]
        {
            let mut options = core::fmt::FormattingOptions::new();
            options.precision(Some(PRECISION as usize));
            self.std_formatter_with_options_adapter(options, |f| d.fmt(f))
        }
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
    fn write_std_args_ref(&mut self, args: &core::fmt::Arguments<'_>) -> Result<(), Self::Error> {
        if let Some(s) = args.as_str() {
            self.write_str(s)
        } else {
            self.std_write_adapter(|w| core::fmt::write(w, *args))
        }
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

            fn write_char(&mut self, c: char) -> core::fmt::Result {
                match self.writer.write_char(c) {
                    Ok(()) => Ok(()),
                    Err(e) => {
                        self.result = Err(e);
                        Err(core::fmt::Error)
                    }
                }
            }

            fn write_fmt(&mut self, args: core::fmt::Arguments<'_>) -> core::fmt::Result {
                match self.writer.write_std_args(args) {
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

    #[cfg(feature = "formatting_options")]
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

    #[cfg(feature = "fmt_internals")]
    #[inline]
    fn std_formatter_adapter(
        &mut self,
        f: impl FnOnce(&mut core::fmt::Formatter) -> core::fmt::Result,
    ) -> Result<(), Self::Error> {
        #[cfg(feature = "formatting_options")]
        {
            self.std_write_adapter(|w| {
                let formatter =
                    &mut core::fmt::Formatter::new(w, core::fmt::FormattingOptions::new());
                f(formatter)
            })
        }
        #[cfg(not(feature = "formatting_options"))]
        {
            self.std_write_adapter(|w| {
                let formatter = &mut core::fmt::Formatter::new(w);
                f(formatter)
            })
        }
    }
}

impl<W> Write for W
where
    W: WriteInfallible + ?Sized,
{
    #[cfg(feature = "never_type")]
    type Error = !;
    #[cfg(not(feature = "never_type"))]
    type Error = ::core::convert::Infallible;

    const IS_LINE_BUFFERED: bool = false;

    #[inline]
    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        self.write_str_infallible(s);
        Ok(())
    }
}

impl Write for core::fmt::Formatter<'_> {
    type Error = core::fmt::Error;

    const IS_LINE_BUFFERED: bool = false;

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

    #[cfg(feature = "formatting_options")]
    #[inline]
    fn std_formatter_with_options_adapter(
        &mut self,
        options: core::fmt::FormattingOptions,
        f: impl FnOnce(&mut core::fmt::Formatter) -> core::fmt::Result,
    ) -> Result<(), Self::Error> {
        let formatter = &mut self.with_options(options);
        f(formatter)
    }

    #[cfg(feature = "fmt_internals")]
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

#[expect(clippy::module_name_repetitions)]
pub trait WriteInfallible {
    fn write_str_infallible(&mut self, s: &str);

    // fn write_infallible<WT>(&mut self, wt: &WT)
    // where
    // 	WT: WriteTo + ?Sized,
    // {
    // 	wt.write_to(self).into_ok();
    // }
}

#[cfg(feature = "std")]
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

pub trait GetWriteInternal: Write {
    #[inline]
    fn get_write_mut_internal(&mut self) -> &mut Self {
        self
    }
    #[inline]
    fn get_write_internal(&mut self) -> &mut Self {
        self
    }
}
impl<W> GetWriteInternal for W where W: Write + ?Sized {}

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

                const IS_LINE_BUFFERED: bool = true;

				#[inline]
				fn write_str(&mut self, s: &str) -> ::core::result::Result<(), Self::Error> {
					::std::io::Write::write_all(self, str::as_bytes(s))
				}

                #[inline]
                fn flush_hint(&mut self) {
                    let _ = ::std::io::Write::flush(self);
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

#[cfg(feature = "std")]
impl_write_flush_for_io_write! {
    std::io::Stdout,
    std::io::StdoutLock<'_>,
    std::io::Stderr,
    std::io::StderrLock<'_>
}

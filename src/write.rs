use crate::writable::{Writable, WritableDebug};

pub trait Write {
    type Error;

    fn write_str(&mut self, s: &str) -> Result<(), Self::Error>;

    #[inline]
    fn writeln_str(&mut self, s: &str) -> Result<(), Self::Error> {
        self.write_str(s)?;
        self.write_str("\n")
    }

    #[inline]
    fn write<WT>(&mut self, wt: &WT) -> Result<(), Self::Error>
    where
        WT: Writable + ?Sized,
    {
        wt.write_to(self)
    }

    fn writeln<WT>(&mut self, wt: &WT) -> Result<(), Self::Error>
    where
        WT: Writable + ?Sized,
    {
        wt.write_to(self)?;
        self.write_str("\n")
    }

    #[inline]
    fn write_debug<WT>(&mut self, wt: &WT) -> Result<(), Self::Error>
    where
        WT: WritableDebug + ?Sized,
    {
        wt.write_to_debug(self)
    }

    fn write_char(&mut self, c: char) -> Result<(), Self::Error> {
        self.write_str(c.encode_utf8(&mut [0; 4]))
    }

    fn write_newline(&mut self) -> Result<(), Self::Error> {
        self.write_str("\n")
    }

    fn write_fmtdisplay<D>(&mut self, d: &D) -> Result<(), Self::Error>
    where
        D: core::fmt::Display + ?Sized,
    {
        fmtwrite_adapter(self, |w| {
            d.fmt(&mut core::fmt::Formatter::new(
                w,
                core::fmt::FormattingOptions::new(),
            ))
        })
    }

    fn write_fmtdebug<D>(&mut self, d: &D) -> Result<(), Self::Error>
    where
        D: core::fmt::Debug + ?Sized,
    {
        fmtwrite_adapter(self, |w| {
            d.fmt(&mut core::fmt::Formatter::new(
                w,
                core::fmt::FormattingOptions::new(),
            ))
        })
    }

    fn write_fmtargs(&mut self, args: core::fmt::Arguments<'_>) -> Result<(), Self::Error> {
        if let Some(s) = args.as_str() {
            self.write_str(s)
        } else {
            fmtwrite_adapter(self, |w| core::fmt::write(w, args))
        }
    }
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

impl Write for core::fmt::Formatter<'_> {
    type Error = core::fmt::Error;

    #[inline]
    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        core::fmt::Formatter::write_str(self, s)
    }

    fn write_fmtdisplay<D>(&mut self, d: &D) -> Result<(), Self::Error>
    where
        D: core::fmt::Display + ?Sized,
    {
        d.fmt(self)
    }

    fn write_fmtdebug<D>(&mut self, d: &D) -> Result<(), Self::Error>
    where
        D: core::fmt::Debug + ?Sized,
    {
        d.fmt(self)
    }

    fn write_fmtargs(&mut self, args: core::fmt::Arguments<'_>) -> Result<(), Self::Error> {
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

fn fmtwrite_adapter<W0>(
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

// pub trait WriteFlush: Write<Error = Self::_Error> {
// 	type _Error;
//
// 	// fn flush(&mut self) -> Result<(), Self::_Error>;
// 	fn flush(&mut self) -> Result<(), Self::Error>;
// }
// default impl<WF> WriteFlush for WF
// where
// 	WF: Write,
// {
// 	type _Error = WF::Error;
//
// 	fn flush(&mut self) -> Result<(), Self::Error> {
// 		Ok(())
// 	}
// }
//
// impl<WF, E> WriteFlush for WF
// where
// 	WF: Write<Error = E> + Flush<Error = E>,
// {
// 	type _Error = E;
//
// 	fn flush(&mut self) -> Result<(), Self::Error> {
// 		Flush::flush(self)
// 	}
// }
// pub trait _WriteFlush: Write<Error = Self::_Error> {
// 	type _Error;
// }
//
// pub trait WriteFlush: _WriteFlush {
// 	fn flush(&mut self) -> Result<(), Self::_Error>;
// }
//
// impl<WF> _WriteFlush for WF
// where
// 	WF: Write,
// {
// 	type _Error = WF::Error;
// }
//
// default impl<WF, E> WriteFlush for WF
// where
// 	WF: Write<Error = E>,
// {
// 	fn flush(&mut self) -> Result<(), E> {
// 		Ok(())
// 	}
// }
//
// impl<WF, E> WriteFlush for WF
// where
// 	WF: Write<Error = E> + Flush<Error = E>,
// {
// 	fn flush(&mut self) -> Result<(), Self::Error> {
// 		Flush::flush(self)
// 	}
// }

pub trait WriteFlush: Write<Error = Self::_Error> + Flush<Error = Self::_Error> {
    type _Error;
}

impl<WF, E> WriteFlush for WF
where
    WF: Write<Error = E> + Flush<Error = E>,
{
    type _Error = E;
}

pub trait FlushHint {
    type Error;
    fn flush_hint(&mut self) -> Result<(), Self::Error>;
    // fn flush_hint(&mut self) {}
}

// impl<W> FlushHint for W
// where
// 	W: Write,
// {
// 	default type Error = W::Error;
//
// 	default fn flush_hint(&mut self) -> Result<(), Self::Error> {
// 		Ok(())
// 	}
// }
// impl<W, E> FlushHint for W
// where
// 	W: Write<Error = E> + Flush<Error = E>,
// {
// 	type Error = <W as Flush>::Error;
//
// 	fn flush_hint(&mut self) -> Result<(), Self::Error> {
// 		self.flush()
// 	}
// }
// pub trait FlushHint {
// 	fn flush_hint(&mut self) {}
// }
//
// default impl<W> FlushHint for W where W: Write {}
// impl<W> FlushHint for W
// where
// 	W: Write + Flush,
// {
// 	fn flush_hint(&mut self) {
// 		self.flush();
// 	}
// }

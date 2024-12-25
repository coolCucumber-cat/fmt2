use crate::{write::Write, write_to::WriteTo};

use core::ops::Deref;

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

trait StaticStrInternal {
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
macro_rules! impl_const_str_for {
	{ $($name:path => $value:expr),* $(,)? } => {
		$(
			impl $crate::write_to::ConstStr for $name {
				const CONST_STR: &str = $value;
			}
		)*
	};
}

#[macro_export]
macro_rules! impl_display_for_str {
	{ $($name:ty),* $(,)? } => {
		$(
			impl ::core::fmt::Display for $name {
				#[inline]
				fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
					::core::fmt::Formatter::write_str(f, $crate::write_to::Str::str(self))
				}
			}
		)*
	};
}

use crate::write_to::FmtAdvanced;

pub trait FmtStr: FmtAdvanced<Target = str> {
    fn fmt_str(&self) -> &str;
}
impl<T> FmtStr for T
where
    T: FmtAdvanced<Target = str> + ?Sized,
{
    #[inline]
    fn fmt_str(&self) -> &str {
        self.fmt_advanced()
    }
}

pub trait ConstStr {
    const CONST_STR: &'static str;
}

pub trait FmtStaticStrImpl {
    fn fmt_static_str_impl(&self) -> &'static str;
}

impl<T> FmtStaticStrImpl for T
where
    T: ConstStr + ?Sized,
{
    #[inline]
    fn fmt_static_str_impl(&self) -> &'static str {
        Self::CONST_STR
    }
}

trait FmtStaticStrInternal {
    fn fmt_static_str_internal(self) -> &'static str;
}

impl<T> FmtStaticStrInternal for &T
where
    T: FmtStaticStrImpl + ?Sized,
{
    #[inline]
    fn fmt_static_str_internal(self) -> &'static str {
        self.fmt_static_str_impl()
    }
}

impl FmtStaticStrInternal for &'static str {
    fn fmt_static_str_internal(self) -> &'static str {
        self
    }
}

pub trait FmtStaticStr<'a> {
    fn fmt_static_str(&'a self) -> &'static str;
}

impl<'a, T> FmtStaticStr<'a> for T
where
    T: 'a,
    &'a T: FmtStaticStrInternal,
{
    #[inline]
    fn fmt_static_str(&'a self) -> &'static str {
        FmtStaticStrInternal::fmt_static_str_internal(self)
    }
}

// pub trait Str {
//     fn str(&self) -> &str;
// }
//
// impl<T> Str for T
// where
//     T: ImplStaticStr + ?Sized,
// {
//     #[inline]
//     fn str(&self) -> &str {
//         self.impl_static_str()
//     }
// }
//
// impl<T> WriteTo for T
// where
//     T: Str + ?Sized,
// {
//     #[inline]
//     fn write_to<W>(&self, w: &mut W) -> Result<(), W::Error>
//     where
//         W: Write + ?Sized,
//     {
//         w.write_str(self.str())
//     }
//
//     #[inline]
//     fn len_hint(&self) -> usize {
//         self.str().len()
//     }
// }
//
// impl Str for str {
//     #[inline]
//     fn str(&self) -> &str {
//         self
//     }
// }

impl<T> FmtAdvanced for [T]
where
    str: transmute_guard::SafeTransmuteRefFrom<[T]>,
{
    type Target = str;
    fn fmt_advanced(&self) -> &Self::Target {
        transmute_guard::safe_transmute_ref(self)
    }
}

#[macro_export]
macro_rules! impl_const_str_for {
	{ $($ty:ty $(=> $value:expr)?),* $(,)? } => {
		$(
            impl $crate::str::ConstStr for $ty {
                const CONST_STR: &str = $crate::impl_const_str_for_get_value_internal! { $ty $(=> $value)? };
            }

            impl $crate::write_to::FmtAdvanced for $ty {
                type Target = str;
                fn fmt_advanced(&self) -> &Self::Target {
                    $crate::str::FmtStaticStrImpl::fmt_static_str_impl(self)
                }
            }
        )*
	};
}

#[macro_export]
#[doc(hidden)]
macro_rules! impl_const_str_for_get_value_internal {
    { $ty:ty => $value:expr } => {
        $value
    };
    { $ty:ty } => {
        ::core::stringify!($ty)
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

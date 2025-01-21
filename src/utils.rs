/// # Safety
/// Only implement this trait if transmuting from `T` to `Self` and vice versa is safe
pub unsafe trait SafeTransmuteFrom<T>: Sized {
    fn safe_transmute_from(value: T) -> Self;
}

unsafe impl<T> SafeTransmuteFrom<T> for T {
    #[inline]
    fn safe_transmute_from(value: T) -> Self {
        value
    }
}

unsafe impl SafeTransmuteFrom<core::ascii::Char> for u8 {
    #[inline]
    fn safe_transmute_from(value: core::ascii::Char) -> Self {
        value.to_u8()
    }
}

unsafe impl<T> SafeTransmuteFrom<&[T]> for &str
where
    core::ascii::Char: SafeTransmuteFrom<T>,
{
    fn safe_transmute_from(value: &[T]) -> Self {
        let b: &[u8] = unsafe { &*(core::ptr::from_ref(value) as *const [u8]) };
        unsafe { core::str::from_utf8_unchecked(b) }
    }
}

pub fn safe_transmute<Src, Dst>(src: Src) -> Dst
where
    Dst: SafeTransmuteFrom<Src>,
{
    Dst::safe_transmute_from(src)
}

pub fn safe_transmute_ref<Src, Dst>(src: &Src) -> &Dst
where
    Dst: SafeTransmuteFrom<Src>,
{
    unsafe { &*(src as *const Src as *const Dst) }
}

pub fn safe_transmute_mut<Src, Dst>(src: &mut Src) -> &mut Dst
where
    Dst: SafeTransmuteFrom<Src>,
{
    unsafe { &mut *(src as *mut Src as *mut Dst) }
}

#[macro_export]
macro_rules! enum_alias {
    {
        $(#[$meta:meta])*
        $vis:vis enum $name:ident: $ty:ty = {$(
            $variant0:ident $(| $variant:ident)*
        )?};
    } => {
        #[repr(u8)]
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        $(#[$meta])*
        enum $name {
            $(
                $variant0 = <$ty>::$variant0 as u8,
                $($variant = <$ty>::$variant as u8, )*
            )?
        }

        impl $name {
            #[inline]
            $vis const fn as_u8(self) -> u8 {
                self as u8
            }

            #[inline]
            $vis const fn into_parent(self) -> ::core::result::Result<$ty, ()> {
                #[cfg(debug_assertions)]
                let self_dev: Self = match value {
                    $(
                        <$name>::$variant0 => <$ty>::$variant0,
                        $(
                            <$name>::$variant => <$ty>::$variant,
                        )*
                    )?
                };
                let self_prod: Self = unsafe { ::core::mem::transmute(value) };
                #[cfg(debug_assertions)]
                {
                    ::core::debug_assert_eq!(self_dev, self_prod, ::core::concat!(::core::stringify!(::core::convert::From<$name> for $ty)));
                }
                self_prod
            }

            #[inline]
            $vis const fn try_from_parent(value: $ty) -> Self {
                match value {
                    $(
                        <$ty>::$variant0 => ::core::result::Result::Ok(<$name>::$variant0),
                        $(
                            <$ty>::$variant => ::core::result::Result::Ok(<$name>::$variant),
                        )*
                        _ => ::core::result::Result::Err(()),
                    )?
                }
            }

        }

        unsafe impl $crate::utils::SafeTransmuteFrom<$name> for $ty {
            #[inline]
            fn safe_transmute_from(value: $name) -> Self {
                unsafe { ::core::mem::transmute(value) }
            }
        }

        impl ::core::convert::From<$name> for $ty {
            #[inline]
            fn from(value: $name) -> Self {
                $name::into_parent(value)
            }
        }

        impl ::core::convert::TryFrom<$ty> for $name {
            type Error = ();
            #[inline]
            fn try_from(value: $ty) -> ::core::result::Result<Self, Self::Error> {
                Self::from_parent(value)
            }
        }
    };
}

// macro_rules! deref {
//     ($value:expr => $ty:path) => {{
//         trait TempDoDeref: $ty {
//             fn temp_do_deref(&self) -> &Self {
//                 self
//             }
//         }
//         impl<T> TempDoDeref for T where T: $ty + ?Sized {}
//         $value.temp_do_deref()
//     }};
// }

#[inline]
#[must_use]
pub fn has_newlines(s: &str) -> bool {
    let v = s.contains('\n');
    #[cfg(debug_assertions)]
    {
        let v2 = s.chars().any(|c| c == '\n');
        debug_assert_eq!(v, v2);
        let count = count_newlines(s);
        debug_assert_eq!(v, count != 0);
    }
    v
}

#[inline]
#[must_use]
pub fn count_newlines(s: &str) -> usize {
    s.chars().filter(|&c| c == '\n').count()
}

#[inline]
#[must_use]
pub fn first_line(s: &str) -> &str {
    let s = first_line_no_debug_assertion(s);
    #[cfg(debug_assertions)]
    {
        debug_assert!(!has_newlines(s));
        debug_assert_eq!(count_newlines(s), 0);
    }
    s
}

#[inline]
#[must_use]
pub fn first_line_no_debug_assertion(s: &str) -> &str {
    let s = match s.find('\n') {
        Some(i) => unsafe { s.get_unchecked(..i) },
        None => s,
    };
    s
}

#[allow(unused)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_line() {
        assert_eq!(first_line_no_debug_assertion("123"), "123");
        assert_eq!(first_line_no_debug_assertion("123\nabc"), "123");
        assert_eq!(first_line_no_debug_assertion("123\n"), "123");
        assert_eq!(first_line_no_debug_assertion("123\nabc\n456"), "123");
        assert_eq!(first_line_no_debug_assertion("123\nabc\n"), "123");
        assert_eq!(first_line_no_debug_assertion("\nabc"), "");
        assert_eq!(first_line_no_debug_assertion("\n"), "");
        assert_eq!(first_line_no_debug_assertion(""), "");
    }
}

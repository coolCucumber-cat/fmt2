/// # Safety
/// Only implement this trait if transmuting from `T` to `Self` and vice versa is safe
pub unsafe trait SafeTransmuteFrom<T>
where
    T: ?Sized,
{
}
unsafe impl<T> SafeTransmuteFrom<T> for T where T: ?Sized {}

/// # Safety
/// Only implement this trait if transmuting from `T` to `Self` and vice versa is safe
pub unsafe trait SafeTransmuteSizedFrom<T>: SafeTransmuteFrom<T> + Sized {
    fn safe_transmute_from(value: T) -> Self;
}
unsafe impl<T> SafeTransmuteSizedFrom<T> for T {
    #[inline]
    fn safe_transmute_from(value: T) -> Self {
        value
    }
}

/// # Safety
/// Only implement this trait if transmuting from `T` to `Self` and vice versa is safe
pub unsafe trait SafeTransmuteRefFrom<T>: SafeTransmuteFrom<T>
where
    T: ?Sized,
{
    fn safe_transmute_ref_from(value: &T) -> &Self;
}
unsafe impl<T, U> SafeTransmuteRefFrom<U> for T
where
    T: SafeTransmuteFrom<U>,
{
    #[inline]
    fn safe_transmute_ref_from(value: &U) -> &Self {
        let u_ptr = core::ptr::from_ref(value);
        let t_ptr: *const Self = u_ptr.cast();
        unsafe { &*t_ptr }
    }
}

/// # Safety
/// Only implement this trait if transmuting from `T` to `Self` and vice versa is safe
pub unsafe trait SafeTransmuteMutFrom<T>: SafeTransmuteFrom<T>
where
    T: ?Sized,
{
    fn safe_transmute_mut_from(value: &mut T) -> &mut Self;
}
unsafe impl<T, U> SafeTransmuteMutFrom<U> for T
where
    T: SafeTransmuteFrom<U>,
{
    #[inline]
    fn safe_transmute_mut_from(value: &mut U) -> &mut Self {
        let u_ptr = core::ptr::from_mut(value);
        let t_ptr: *mut Self = u_ptr.cast();
        unsafe { &mut *t_ptr }
    }
}

unsafe impl<U> SafeTransmuteFrom<[U]> for str where core::ascii::Char: SafeTransmuteSizedFrom<U> {}
unsafe impl<T> SafeTransmuteRefFrom<[T]> for str
where
    core::ascii::Char: SafeTransmuteSizedFrom<T>,
{
    #[inline]
    fn safe_transmute_ref_from(value: &[T]) -> &Self {
        let u_ptr = core::ptr::from_ref(value);
        let s_ptr = u_ptr as *const [u8];
        let s: &[u8] = unsafe { &*(s_ptr) };
        unsafe { core::str::from_utf8_unchecked(s) }
    }
}
unsafe impl<U> SafeTransmuteMutFrom<[U]> for str
where
    core::ascii::Char: SafeTransmuteSizedFrom<U>,
{
    #[inline]
    fn safe_transmute_mut_from(value: &mut [U]) -> &mut Self {
        let u_ptr = core::ptr::from_mut(value);
        let s_ptr = u_ptr as *mut [u8];
        let s: &mut [u8] = unsafe { &mut *(s_ptr) };
        unsafe { core::str::from_utf8_unchecked_mut(s) }
    }
}

unsafe impl SafeTransmuteFrom<core::ascii::Char> for u8 {}
unsafe impl SafeTransmuteSizedFrom<core::ascii::Char> for u8 {
    #[inline]
    fn safe_transmute_from(value: core::ascii::Char) -> Self {
        value.to_u8()
    }
}

#[inline]
pub fn safe_transmute<Src, Dst>(src: Src) -> Dst
where
    Dst: SafeTransmuteSizedFrom<Src>,
{
    Dst::safe_transmute_from(src)
}

#[inline]
pub fn safe_transmute_ref<Src, Dst>(src: &Src) -> &Dst
where
    Dst: SafeTransmuteRefFrom<Src> + ?Sized,
    Src: ?Sized,
{
    Dst::safe_transmute_ref_from(src)
}

#[inline]
pub fn safe_transmute_mut<Src, Dst>(src: &mut Src) -> &mut Dst
where
    Dst: SafeTransmuteMutFrom<Src> + ?Sized,
    Src: ?Sized,
{
    Dst::safe_transmute_mut_from(src)
}

#[inline]
pub fn safe_transmute_slice<Src, Dst>(src: &[Src]) -> &[Dst]
where
    Dst: SafeTransmuteFrom<Src>,
{
    let u_ptr = core::ptr::from_ref(src);
    let t_ptr = u_ptr as *const [Dst];
    unsafe { &*t_ptr }
}

#[inline]
pub fn safe_transmute_slice_mut<Src, Dst>(src: &mut [Src]) -> &mut [Dst]
where
    Dst: SafeTransmuteFrom<Src>,
{
    let u_ptr = core::ptr::from_mut(src);
    let t_ptr = u_ptr as *mut [Dst];
    unsafe { &mut *t_ptr }
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
            $vis const fn into_parent(self) -> $ty {
                unsafe { ::core::mem::transmute(self) }
            }

            #[inline]
            $vis const fn try_from_parent(value: $ty) -> ::core::result::Result<Self, ()> {
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

        unsafe impl $crate::utils::SafeTransmuteFrom<$name> for $ty {}
        unsafe impl $crate::utils::SafeTransmuteSizedFrom<$name> for $ty {
            #[inline]
            fn safe_transmute_from(value: $name) -> Self {
                unsafe { ::core::mem::transmute(value) }
            }
        }

        impl ::core::convert::From<$name> for $ty {
            #[inline]
            fn from(value: $name) -> Self {
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
        }

        impl ::core::convert::TryFrom<$ty> for $name {
            type Error = ();
            #[inline]
            fn try_from(value: $ty) -> ::core::result::Result<Self, Self::Error> {
                Self::try_from_parent(value)
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
    use core::ascii::Char;

    use crate::write_to::FmtAdvanced;

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

    #[test]
    fn test_safe_transmute() {
        assert_eq!(
            safe_transmute_ref::<[Char], str>(&[Char::CapitalA, Char::Digit5, Char::SmallI]),
            "A5i"
        );
        assert_eq!(safe_transmute_ref::<Char, u8>(&Char::CapitalE), &b'E');
        assert_eq!(
            safe_transmute_mut::<[Char], str>(&mut [Char::CapitalA, Char::Digit5, Char::SmallI]),
            "A5i"
        );
        assert_eq!(
            safe_transmute_mut::<Char, u8>(&mut Char::CapitalE),
            &mut b'E'
        );
        assert_eq!(
            [Char::CapitalC, Char::Digit3, Char::Colon].fmt_advanced(),
            "C3:"
        );
    }
}

/// # Safety
/// Only implement this trait if transmuting from `T` to `Self` and vice versa is safe
pub unsafe trait ImplSafeTransmuteFrom<T>: Sized {
    fn impl_safe_transmute_from(value: T) -> Self;
}

// unsafe impl<T> ImplSafeTransmuteFrom<T> for T {
//     fn impl_safe_transmute_from(value: T) -> Self {
//         value
//     }
// }

unsafe impl<T, U> ImplSafeTransmuteFrom<&[T]> for &[U]
where
    U: ImplSafeTransmuteFrom<T>,
{
    fn impl_safe_transmute_from(value: &[T]) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}

/// # Safety
/// Only implement this trait if transmuting from `T` to `Self` and vice versa is safe
pub unsafe trait SafeTransmuteFrom<T>: Sized {
    fn safe_transmute_from(value: T) -> Self;
}

unsafe impl<T> SafeTransmuteFrom<T> for T {
    fn safe_transmute_from(value: T) -> Self {
        value
    }
}

// unsafe impl<T, U> SafeTransmuteFrom<&[T]> for &[U]
// where
//     U: SafeTransmuteFrom<T>,
// {
//     fn safe_transmute_from(value: &[T]) -> Self {
//         unsafe { ::core::mem::transmute(value) }
//     }
// }
//
// #[macro_export]
// macro_rules! sussy {
//     () => {};
// }
//
// #[macro_export]
// macro_rules! enum_alias {
//     {
//         $(#[$meta:meta])*
//         enum $name:ident: $ty:ty = $({
//             $variant0:ident $(| $variant:ident)*
//         })?;
//     } => {
//         #[repr(u8)]
//         #[derive(Debug, Copy, Clone, PartialEq, Eq)]
//         $(#[$meta])*
//         enum $name {
//             $(
//                 $variant0 = <$ty>::$variant0 as u8,
//                 $($variant = <$ty>::$variant as u8, )*
//             )?
//         }
//
//         unsafe impl $crate::utils::ImplSafeTransmuteFrom<$name> for $ty {
//             #[inline]
//             fn impl_safe_transmute_from(value: $name) -> Self {
//                 unsafe { ::core::mem::transmute(value) }
//             }
//         }
//
//         impl ::core::convert::From<$name> for $ty {
//             #[inline]
//             fn from(value: $name) -> Self {
//                 #[cfg(debug_assertions)]
//                 let self_dev: Self = match value {
//                     $(
//                         <$name>::$variant0 => <$ty>::$variant0,
//                         $(
//                             <$name>::$variant => <$ty>::$variant,
//                         )*
//                     )?
//                 };
//                 let self_prod: Self = unsafe { ::core::mem::transmute(value) };
//                 #[cfg(debug_assertions)]
//                 {
//                     ::core::debug_assert_eq!(self_dev, self_prod, ::core::concat!(::core::stringify!(::core::convert::From<$name> for $ty)));
//                 }
//                 self_prod
//             }
//         }
//
//         impl ::core::convert::TryFrom<$ty> for $name {
//             type Error = ();
//             #[inline]
//             fn try_from(value: $ty) -> Result<Self, Self::Error> {
//                 match value {
//                     $(
//                         <$ty>::$variant0 => ::core::result::Result::Ok(<$name>::$variant0),
//                         $(
//                             <$ty>::$variant => ::core::result::Result::Ok(<$name>::$variant),
//                         )*
//                         _ => ::core::result::Result::Err(()),
//                     )?
//                 }
//             }
//         }
//     };
// }

// #[derive(Debug, PartialEq, Eq, Clone, Copy)]
// enum X {
//     A,
//     B,
//     C,
// }
//
// enum_alias! { #[doc(alias = "sussy")] #[doc = "alias"] enum Y: X = { C | B }; }

#[allow(unused)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}

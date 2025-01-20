use crate::write::Write;

pub trait WriteTo {
    const ENDS_IN_NEWLINE: bool = false;
    const MIN_SIZE: usize = 0;

    fn write_to<W>(&self, w: &mut W) -> Result<(), W::Error>
    where
        W: Write + ?Sized;

    #[inline]
    fn len_hint(&self) -> usize {
        0
    }
}

impl WriteTo for str {
    fn write_to<W>(&self, w: &mut W) -> Result<(), W::Error>
    where
        W: Write + ?Sized,
    {
        w.write_str(self)
    }

    fn len_hint(&self) -> usize {
        self.len()
    }
}

impl<T> WriteTo for Debug<[T]>
where
    T: FmtDebug,
{
    crate::fmt! { [s] => "[" @..(s.0 => |e| {e;?} ", ") "]" }
}

pub trait FmtAdvanced {
    type Target: WriteTo + ?Sized;
    fn fmt_advanced(&self) -> &Self::Target;
}

impl<T> FmtAdvanced for T
where
    T: WriteTo + ?Sized,
{
    type Target = Self;
    #[inline]
    fn fmt_advanced(&self) -> &Self::Target {
        self
    }
}

pub trait Fmt {
    fn fmt(&self) -> &(impl WriteTo + ?Sized);
}

impl<T> Fmt for T
where
    T: FmtAdvanced + ?Sized,
{
    #[inline]
    fn fmt(&self) -> &(impl WriteTo + ?Sized) {
        self.fmt_advanced()
    }
}

// impl<I, T> WriteTo for I
// where
//     I: Iterator<Item = &T> + ?Sized,
//     T: WriteTo + ?Sized,
// {
//     const ENDS_IN_NEWLINE: bool = T::ENDS_IN_NEWLINE;
//     fn write_to<W>(&self, w: &mut W) -> Result<(), W::Error>
//     where
//         W: Write + ?Sized,
//     {
//         for v in self {
//             w.write(v)
//         }
//         Ok(())
//     }
// }

#[macro_export]
macro_rules! declare_fmt_wrapper_struct {
    { $Struct:ident $Trait:ident $fn:ident $(, $($($rest:tt)+)?)? } => {
        pub struct $Struct<T>(T)
        where
            T: ?Sized,
            Self: $crate::write_to::WriteTo;

        pub trait $Trait {
            fn $fn(&self) -> &(impl $crate::write_to::WriteTo + ?Sized);
        }

        impl<T> $Trait for T
        where
            T: ?Sized,
            $Struct<T>: $crate::write_to::WriteTo,
        {
            #[inline]
            fn $fn(&self) -> &(impl $crate::write_to::WriteTo + ?Sized) {
                unsafe { &*(self as *const Self as *const $Struct<Self>) }
            }
        }

        $($($crate::declare_fmt_wrapper_struct! { $($rest)+ })?)?
    };
    { $Struct:ident <$(const $CONST:ident : $ConstType:ty),* $(,)?> $Trait:ident $fn:ident $(, $($($rest:tt)+)?)? } => {
        pub struct $Struct<T $(, const $CONST: $ConstType)*>(T)
        where
            T: ?Sized,
            $Struct<T $(, { $CONST })*>: $crate::write_to::WriteTo;

        pub trait $Trait {
            fn $fn<$(const $CONST : $ConstType),*>(&self) -> &$Struct<Self $(, { $CONST })*>
            where
                $Struct<Self $(, { $CONST })*>: $crate::write_to::WriteTo;
        }

        impl<T> $Trait for T
        where
            T: ?Sized,
        {
            #[inline]
            fn $fn<$(const $CONST : $ConstType),*>(&self) -> &$Struct<Self $(, { $CONST })*>
            where
                $Struct<Self $(, { $CONST })*>: $crate::write_to::WriteTo
            {
                unsafe { &*(self as *const Self as *const $Struct<Self $(, $CONST)*>) }
            }
        }
//         pub struct $Struct<T $($(, const $CONST: $ConstType)*)?>(T)
//         where
//             T: ?Sized,
//             Self: $crate::write_to::WriteTo;
//
//         pub trait $Trait $(<$(const $CONST: $ConstType),*>)? {
//             fn $fn(&self) -> &(impl $crate::write_to::WriteTo + ?Sized);
//         }
//
//         impl<T $($(, const $CONST: $ConstType)*)?> $Trait $(<$($CONST),*>)? for T
//             where
//             T: ?Sized,
//             $Struct<T $($(, { $CONST })*)?>: $crate::write_to::WriteTo,
//         {
//             #[inline]
//             fn $fn(&self) -> &(impl $crate::write_to::WriteTo + ?Sized) {
//                 unsafe { &*(self as *const Self as *const $Struct<Self $($(, $CONST)*)?>) }
//             }
//         }

        $($($crate::declare_fmt_wrapper_struct! { $($rest)+ })?)?

    };
}

macro_rules! declare_std_write_to_wrapper_struct_internal {
    { $($Struct:ident $(<$(const $CONST:ident : $ConstType:ty),* $(,)?>)? $Trait:ident $fn:ident => $StdTrait:ident $write_fn:ident),* $(,)? } => {
        $(
            $crate::declare_fmt_wrapper_struct! { $Struct $(<$(const $CONST : $ConstType),*>)? $Trait $fn }

            impl<T $($(, const $CONST : $ConstType)*)?> $crate::write_to::WriteTo for $Struct<T $($(, { $CONST })*)?>
            where
                T: ::core::fmt::$StdTrait + ?Sized,
            {
                #[inline]
                fn write_to<W>(&self, w: &mut W) -> ::core::result::Result<(), W::Error>
                where
                    W: $crate::write::Write + ?Sized,
                {
                    $crate::write::Write::$write_fn::<T $($(, { $CONST })*)?>(w, &self.0)
                }
            }
        )*
    };
}

macro_rules! impl_fmt_trait_internal {
    { $StdTrait:ident $std_fn:ident => $Trait:ident $fn:ident => $($ty:ty)*} => {
        $(
            impl $Trait for $ty {
                #[inline]
                fn $fn(&self) -> &(impl $crate::write_to::WriteTo + ?Sized) {
                    $StdTrait::$std_fn(self)
                }
            }
        )*
    };
}

declare_fmt_wrapper_struct! {
    Debug   FmtDebug    fmt_debug,
    Binary  FmtBinary   fmt_binary,
    Octal   FmtOctal    fmt_octal,
    Hex     FmtHex      fmt_hex,
    Precision<const PRECISION: u8> FmtPrecision fmt_precision,
    Iterator FmtIterator fmt_iterator,
}

declare_std_write_to_wrapper_struct_internal! {
    StdDisplay  FmtStdDisplay   fmt_std_display => Display  write_std_display,
    StdDebug    FmtStdDebug     fmt_std_debug   => Debug    write_std_debug,
    StdBinary   FmtStdBinary    fmt_std_binary  => Binary   write_std_binary,
    StdOctal    FmtStdOctal     fmt_std_octal   => Octal    write_std_octal,
    StdHex      FmtStdHex       fmt_std_hex     => UpperHex write_std_upper_hex,
    StdPrecision<const PRECISION: u8>   FmtStdPrecision fmt_std_precision   => Display write_std_precision,
}

impl_fmt_trait_internal! { FmtStdDisplay    fmt_std_display => Fmt          fmt         => usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }
impl_fmt_trait_internal! { FmtStdDebug      fmt_std_debug   => FmtDebug     fmt_debug   => usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }
impl_fmt_trait_internal! { FmtStdBinary     fmt_std_binary  => FmtBinary    fmt_binary  => usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
impl_fmt_trait_internal! { FmtStdOctal      fmt_std_octal   => FmtOctal     fmt_octal   => usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
impl_fmt_trait_internal! { FmtStdHex        fmt_std_hex     => FmtHex       fmt_hex     => usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }

//
// impl<'t, I, T> WriteTo for Iterator<I>
// where
//     I: core::iter::Iterator<Item = &'t T> + Clone,
//     T: WriteTo + ?Sized + 't,
// {
//     const ENDS_IN_NEWLINE: bool = T::ENDS_IN_NEWLINE;
//
//     fn write_to<W>(&self, w: &mut W) -> Result<(), W::Error>
//     where
//         W: Write + ?Sized,
//     {
//         for t in self.0.clone() {
//             t.write_to(w)?;
//         }
//         Ok(())
//     }
// }

impl<'t, I, T> WriteTo for Iterator<I>
where
    I: core::iter::Iterator<Item = &'t T> + Clone,
    T: WriteTo + ?Sized + 't,
{
    const ENDS_IN_NEWLINE: bool = T::ENDS_IN_NEWLINE;

    #[inline]
    fn write_to<W>(&self, w: &mut W) -> Result<(), W::Error>
    where
        W: Write + ?Sized,
    {
        for t in self.0.clone() {
            t.write_to(w)?;
        }
        Ok(())
    }

    #[inline]
    fn len_hint(&self) -> usize {
        self.0.size_hint().0 * T::MIN_SIZE
    }
}

impl WriteTo for core::fmt::Arguments<'_> {
    fn write_to<W>(&self, w: &mut W) -> Result<(), W::Error>
    where
        W: Write + ?Sized,
    {
        w.write_std_args_ref(self)
    }
}

impl WriteTo for Debug<str> {
    fn write_to<W>(&self, w: &mut W) -> Result<(), W::Error>
    where
        W: Write + ?Sized,
    {
        w.write_str(&self.0)
    }
}

// impl FmtDebug for bool {
//     type Target = Self;
//     fn fmt_debug(&self) -> &Self::Target {
//         self
//     }
// }

// pub trait WriteToFor<W>
// where
//     W: crate::write::Write + ?Sized,
// {
//     fn write_to_for(&self, w: &mut W) -> Result<(), W::Error>;
// }
//
// impl<F, W> WriteToFor<W> for F
// where
//     F: Fn(&mut W) -> Result<(), W::Error>,
//     W: crate::write::Write,
// {
//     fn write_to_for(&self, w: &mut W) -> Result<(), W::Error> {
//         self(w)
//     }
// }

pub trait ToString {
    fn to_string(&self) -> String;
}

impl<T> ToString for T
where
    T: Fmt + ?Sized,
{
    fn to_string(&self) -> String {
        let wt = self.fmt();
        let mut s = String::with_capacity(wt.len_hint());
        s.write(wt).into_ok();
        s
    }
}

#[derive(Clone, Copy)]
pub struct WithFmtAdvanced<'a, T, U = str>
where
    U: WriteTo + ?Sized,
{
    pub value: T,
    pub fmt: &'a U,
}

impl<'a, T, U> WithFmtAdvanced<'a, T, U>
where
    U: WriteTo + ?Sized,
{
    pub const fn new(value: T, str: &'a U) -> Self {
        Self { value, fmt: str }
    }

    pub fn map_value<V>(self, f: impl FnOnce(T) -> V) -> WithFmtAdvanced<'a, V, U> {
        WithFmtAdvanced {
            value: f(self.value),
            fmt: self.fmt,
        }
    }

    pub fn replace_value<V>(self, value: V) -> WithFmtAdvanced<'a, V, U> {
        WithFmtAdvanced {
            value,
            fmt: self.fmt,
        }
    }
}

impl<T, U> FmtAdvanced for WithFmtAdvanced<'_, T, U>
where
    U: WriteTo + ?Sized,
{
    type Target = U;
    fn fmt_advanced(&self) -> &Self::Target {
        self.fmt.fmt_advanced()
    }
}

impl<T0, T1, U> AsRef<T1> for WithFmtAdvanced<'_, T0, U>
where
    T0: AsRef<T1>,
    T1: ?Sized,
    U: WriteTo + ?Sized,
{
    #[inline]
    fn as_ref(&self) -> &T1 {
        self.value.as_ref()
    }
}

impl<T, U> core::ops::Deref for WithFmtAdvanced<'_, T, U>
where
    U: WriteTo + ?Sized,
{
    type Target = Self;

    fn deref(&self) -> &Self::Target {
        self
    }
}

#[macro_export]
macro_rules! impl_write_to_for_std_display {
	{ $($ty:ident $write_to_fn:ident),* $(,)? } => {
		$(
			impl $crate::write_to::WriteTo for $name {
				#[inline]
				fn write_to<W>(&self, w: &mut W) -> Result<(), W::Error>
					where
						W: $crate::write::Write + ?Sized {
					$crate::write::Write::write_std_display(w, self)
				}
			}
		)*
	};
}

#[macro_export]
macro_rules! impl_std_display_for_write_to {
	{ $($name:ty),* $(,)? } => {
		$(
			impl ::core::fmt::Display for $name {
				#[inline]
				fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
					$crate::write::Write::write(f, self)
				}
			}
		)*
	};
}

#[allow(
    clippy::allow_attributes,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::missing_const_for_fn,
    unused_variables,
    unused_imports
)]
#[cfg(test)]
mod tests {
    use core::borrow::Borrow;

    use crate::{str::FmtStaticStr, write_to::WriteTo};

    use super::{FmtAdvanced, ToString};

    #[test]
    fn borrow() {
        let k = {
            let s = "123";
            // let s = String::new();
            s.fmt_advanced()
        };
        ToString::to_string(k);
        let k = {
            let s = true;
            s.fmt_static_str()
        };
        ToString::to_string(k);
    }

    #[test]
    fn write() {
        use crate::{
            write::{Flush, Write, WriteInfallible},
            write_to::WriteTo,
        };

        struct Test(bool);

        impl WriteTo for Test {
            fn write_to<W>(&self, w: &mut W) -> Result<(), W::Error>
            where
                W: Write + ?Sized,
            {
                w.write_str(self.0.fmt_advanced())
            }
        }

        let mut s = String::new();
        s.write(&Test(true)).into_ok();
        assert_eq!(s, "true");

        let mut s = String::new();
        s.write(&Test(false)).into_ok();
        assert_eq!(s, "false");

        let mut s = String::new();
        Test(true).write_to(&mut s).into_ok();
        assert_eq!(s, "true");

        let mut s = String::new();
        Test(false).write_to(&mut s).into_ok();
        assert_eq!(s, "false");

        let mut s = String::new();
        s.write_str_infallible("123456");
        assert_eq!(s, "123456");

        let mut s = String::new();
        WriteInfallible::write_str_infallible(&mut s, "123abc");
        assert_eq!(s, "123abc");

        let mut s = String::new();
        "123abc".write_to(&mut s).into_ok();
        assert_eq!(s, "123abc");
    }
}

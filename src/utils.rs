use core::ops::Deref;

pub trait DerefForWritableFmt {
    type Target: ?Sized;

    #[must_use]
    fn deref_for_writable(&self) -> &Self::Target;
}

pub trait DerefForWritableMut<'a> {
    type Target: ?Sized;

    #[must_use]
    fn deref_for_writable(self) -> &'a Self::Target;
}

impl<'a, T: ?Sized, U: ?Sized> DerefForWritableMut<'a> for &'a mut T
where
    T: Deref<Target = U>,
{
    type Target = U;

    fn deref_for_writable(self) -> &'a Self::Target {
        Deref::deref(self)
    }
}

pub trait DerefForWritable<'a> {
    type Target: ?Sized;

    #[must_use]
    fn deref_for_writable(self) -> &'a Self::Target;
}

impl<'a, T: ?Sized, U: ?Sized> DerefForWritable<'a> for &'a T
where
    T: Deref<Target = U>,
{
    type Target = U;

    fn deref_for_writable(self) -> &'a Self::Target {
        Deref::deref(self)
    }
}

// pub trait DerefForWritable {
//     type Target: ?Sized;
//
//     #[must_use]
//     fn deref_for_writable(&self) -> &Self::Target;
// }
//
// impl<T: ?Sized, U: ?Sized> DerefForWritable for T
// where
//     T: Deref<Target = U>,
// {
//     type Target = U;
//
//     fn deref_for_writable(&self) -> &Self::Target {
//         Deref::deref(self)
//     }
// }

#[allow(unused)]
fn test() {
    let a: String = String::new();
    let b: &String = &String::new();
    let c: &mut String = &mut String::new();

    let a = a.deref_for_writable();
    let b = b.deref_for_writable();
    let c = c.deref_for_writable();
    let c = c.deref_for_writable();
}

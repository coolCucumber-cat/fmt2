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

// pub trait DerefForWritable<'a> {
//     type Target: ?Sized;
//
//     #[must_use]
//     fn deref_for_writable(self) -> &'a Self::Target;
// }
//
// impl<'a, T: ?Sized, U: ?Sized> DerefForWritable<'a> for &'a T
// where
//     T: Deref<Target = U>,
// {
//     type Target = U;
//
//     fn deref_for_writable(self) -> &'a Self::Target {
//         Deref::deref(self)
//     }
// }

pub trait DerefForWritable {
    type Target: ?Sized;

    #[must_use]
    fn deref_for_writable(&self) -> &Self::Target;
}

impl<T: ?Sized, U: ?Sized> DerefForWritable for T
where
    T: Deref<Target = U>,
{
    type Target = U;

    fn deref_for_writable(&self) -> &Self::Target {
        Deref::deref(self)
    }
}

pub trait DerefForWritable2 {
    type Target: ?Sized;

    #[must_use]
    fn deref_for_writable2(&self) -> &Self::Target;
}

impl<T: ?Sized> DerefForWritable2 for T {
    type Target = T;

    fn deref_for_writable2(&self) -> &Self::Target {
        self
    }
}

#[allow(unused)]
fn test() {
    fn takes_str(s: &str) {}

    let a: String = String::new();
    let b: &String = &String::new();
    let c: &mut String = &mut String::new();

    let a = a.deref_for_writable();
    let b = b.deref_for_writable();
    let c = c.deref_for_writable();
    let c = c.deref_for_writable();

    {
        let i = 32;
        let i0 = i.deref_for_writable2();

        let s = "123";
        let s0 = s.deref_for_writable2();

        let s = String::new();
        let s0 = (*s).deref_for_writable2();

        let s = &mut String::new();
        let s0 = (**s).deref_for_writable2();
    }
}

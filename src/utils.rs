use core::ops::Deref;

pub trait DerefIgnoreMutForMut<'a> {
    type Target: ?Sized;

    #[must_use]
    fn deref_ignore_mut(self) -> &'a Self::Target;
}

impl<'a, T: ?Sized, U: ?Sized> DerefIgnoreMutForMut<'a> for &'a mut T
where
    T: Deref<Target = U>,
{
    type Target = U;

    fn deref_ignore_mut(self) -> &'a U {
        Deref::deref(self)
    }
}

pub trait DerefIgnoreMut {
    type Target: ?Sized;

    #[must_use]
    fn deref_ignore_mut(&self) -> &Self::Target;
}

impl<T: ?Sized, U: ?Sized> DerefIgnoreMut for T
where
    T: Deref<Target = U>,
{
    type Target = U;

    fn deref_ignore_mut(&self) -> &U {
        Deref::deref(self)
    }
}

#[allow(unused)]
fn test() {
    let a: String = String::new();
    let b: &String = &String::new();
    let c: &mut String = &mut String::new();

    let a = a.deref_ignore_mut();
    let b = b.deref_ignore_mut();
    let c = c.deref_ignore_mut();
    let c = c.deref_ignore_mut();
}

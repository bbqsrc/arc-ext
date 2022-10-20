use std::{
    marker::PhantomData,
    ops::Deref,
    pin::Pin,
    sync::Arc,
};

pub struct ArcProject<'a, 'b, T, U>
where
    T: ?Sized + Unpin,
{
    value: *const U,
    inner: Pin<Arc<T>>,
    _lifetime: PhantomData<&'a &'b ()>,
}

impl<'a, 'b, T, U> ArcProject<'a, 'b, T, U>
where
    'a: 'b,
    T: ?Sized + Unpin + 'a,
    U: 'b,
{
    pub fn new<F: Fn(&'a Pin<Arc<T>>) -> &'b U>(inner: Arc<T>, deref_fn: F) -> Self {
        let mut out = Self {
            value: std::ptr::null(),
            inner: Pin::new(inner),
            _lifetime: PhantomData,
        };

        out.value = deref_fn(unsafe { std::mem::transmute(&out.inner) });
        out
    }
}

impl<'a, 'b, T, U> Deref for ArcProject<'a, 'b, T, U>
where
    'a: 'b,
    T: ?Sized + Unpin + 'a,
    U: 'b,
{
    type Target = U;

    fn deref(&self) -> &'b Self::Target {
        unsafe { &*self.value }
    }
}

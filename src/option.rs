use std::{
    marker::{PhantomData},
    pin::Pin,
    sync::Arc,
};

pub struct ArcProjectOption<'top, 'inner, T, U>
where
    'top: 'inner,
    T: ?Sized + Unpin,
    U: ?Sized,
{
    opt: Option<*const U>,
    inner: Pin<Arc<T>>,
    _lifetime: PhantomData<&'top &'inner ()>,
}

impl<'top, 'inner, T, U> ArcProjectOption<'top, 'inner, T, U>
where
    T: ?Sized + Unpin + 'top,
    U: ?Sized + 'inner,
{
    pub fn new<F: Fn(&'top Pin<Arc<T>>) -> Option<&'inner U>>(inner: Arc<T>, deref_fn: F) -> Self {
        let mut out = Self {
            opt: None,
            inner: Pin::new(inner),
            _lifetime: PhantomData,
        };

        out.opt = deref_fn(unsafe { std::mem::transmute(&out.inner) }).map(|x| x as _);
        out
    }

    pub fn as_option(&'top self) -> Option<&'inner U> {
        self.opt.map(|x| unsafe { &*x })
    }
}

use std::{ops::Deref, ptr::NonNull, slice::SliceIndex};

use crate::cell::Cell;

struct RcInner<T> {
    value: T,
    refcount: Cell<usize>,
}
pub struct Rc<T> {
    inner: NonNull<RcInner<T>>,
}

impl<T> Rc<T> {
    fn new(&self, value: T) -> Self {
        let inner = Box::new(RcInner {
            value: value,
            refcount: Cell::new(1),
        });

        Rc {
            inner: unsafe { NonNull::new_unchecked(Box::into_raw(inner)) },
        }
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        let inner = unsafe { self.inner.as_ref() };
        let c = inner.refcount.get();
        inner.refcount.set(c + 1);
        Rc { inner: self.inner }
    }
}

impl<T> Deref for Rc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &unsafe { self.inner.as_ref() }.value
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        let inner = unsafe { self.inner.as_ref() };
        let c = inner.refcount.get();
        if *c == 1 {
            drop(inner);
            let _ = unsafe {
                Box::from_raw(self.inner.as_ptr());
            };
        } else {
            inner.refcount.set(c - 1);
        }
    }
}

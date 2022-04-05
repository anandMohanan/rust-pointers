use crate::cell::Cell;
use std::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
};

#[derive(Clone, Copy)]
enum RefState {
    Shared(isize),
    Unshared,
    Exclusive,
}

pub struct RefCell<T> {
    value: UnsafeCell<T>,
    state: Cell<RefState>,
}

pub struct Ref<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

pub struct RefMut<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

impl<T> Drop for Ref<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Shared(1) => self.refcell.state.set(RefState::Unshared),
            RefState::Shared(n) => self.refcell.state.set(RefState::Shared(n - 1)),
            RefState::Unshared => unreachable!(),
            RefState::Exclusive => unreachable!(),
        }
    }
}

impl<T> Deref for Ref<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.refcell.value.get() }
    }
}

impl<T> Deref for RefMut<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.refcell.value.get() }
    }
}

impl<T> DerefMut for RefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.refcell.value.get() }
    }
}

impl<T> Drop for RefMut<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Shared(_) => unreachable!(),
            RefState::Unshared => unreachable!(),
            RefState::Exclusive => self.refcell.state.set(RefState::Unshared),
        }
    }
}
impl<T> RefCell<T> {
    fn new(value: T) -> RefCell<T> {
        RefCell {
            value: UnsafeCell::new(value),
            state: Cell::new(RefState::Unshared),
        }
    }

    fn borrow(&self) -> Option<Ref<'_, T>> {
        match self.state.get() {
            RefState::Shared(n) => {
                self.state.set(RefState::Shared(n + 1));
                Some(Ref { refcell: self })
            }
            RefState::Unshared => {
                self.state.set(RefState::Shared(1));
                Some(Ref { refcell: self })
            }
            RefState::Exclusive => None,
        }
    }

    fn borrow_mut(&self) -> Option<RefMut<'_, T>> {
        match self.state.get() {
            RefState::Unshared => {
                self.state.set(RefState::Exclusive);
                Some(RefMut { refcell: self })
            }
            _ => None,
        }
    }
}

use std::cell::UnsafeCell;

#[derive(Debug)]
pub struct Cell<T> {
    value: UnsafeCell<T>,
}

unsafe impl<T> Sync for Cell<T> {}

impl<T> Cell<T> {
    pub fn new(value: T) -> Cell<T> {
        Cell {
            value: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, value: T) {
        unsafe {
            *self.value.get() = value;
        }
    }

    pub fn get(&self) -> &T {
        unsafe { &*self.value.get() }
    }
}

#[cfg(test)]
mod test {
    use super::Cell;

    #[test]
    fn bad() {
        let x = Cell::new(987);
        eprint!("{:?}", x);
        let first = x.get();
        eprintln!("{first}");
        x.set(45);
        x.set(897);
        eprintln!("aaa {first}");
    }
}

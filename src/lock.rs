use std::sync::{Arc, Mutex};
use std::rc::Rc;
use std::cell::RefCell;


pub(crate) trait LockFamily {
    type Lock<T>: WithInner;
    fn new<T>(value: T) -> Self::Lock<T>;
}

pub(crate) trait WithInner {
    type In;

    fn with_inner<F, U: Sized>(&self, f: F) -> U
    where
        F: FnOnce(&mut Self::In) -> U;
}

impl<T> WithInner for Arc<Mutex<T>> {
    type In = T;

    fn with_inner<F, U: Sized>(&self, f: F) -> U
    where
        F: FnOnce(&mut T) -> U,
    {
        let mut t = self.lock().unwrap();
        let u = f(&mut *t);
        u
    }
}

pub(crate) struct ArcMutexFamily;

impl LockFamily for ArcMutexFamily {
    type Lock<T> = Arc<Mutex<T>>;
    fn new<T>(value: T) -> Self::Lock<T> {
        Arc::new(Mutex::new(value))
    }
}

pub(crate) struct RcRefCellFamily;

impl LockFamily for RcRefCellFamily {
    type Lock<T> = Rc<RefCell<T>>;
    fn new<T>(value: T) -> Self::Lock<T> {
        Rc::new(RefCell::new(value))
    }
}

impl<T> WithInner for Rc<RefCell<T>> {
    type In = T;

    fn with_inner<F, U: Sized>(&self, f: F) -> U
    where
        F: FnOnce(&mut T) -> U,
    {
        let mut t = self.borrow_mut();
        let u = f(&mut *t);
        u
    }
}
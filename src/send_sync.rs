#![allow(dead_code)]

use std::marker::PhantomData;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;

pub trait Shared<T>: Clone + Deref<Target = T> {
    fn new(value: T) -> Self;
    fn try_get_mut(this: &mut Self) -> Option<&mut T>;
}

impl<T> Shared<T> for Rc<T> {
    fn new(value: T) -> Self {
        Rc::new(value)
    }

    fn try_get_mut(this: &mut Self) -> Option<&mut T> {
        Rc::get_mut(this)
    }
}

impl<T> Shared<T> for Arc<T> {
    fn new(value: T) -> Self {
        Arc::new(value)
    }

    fn try_get_mut(this: &mut Self) -> Option<&mut T> {
        Arc::get_mut(this)
    }
}

pub struct SharedPtr<T, P: Shared<T>> {
    inner: P,
    _marker: PhantomData<T>,
}

impl<T, P: Shared<T>> SharedPtr<T, P> {
    pub fn new(value: T) -> Self {
        Self {
            inner: P::new(value),
            _marker: PhantomData,
        }
    }

    pub fn get(&self) -> &T {
        &*self.inner
    }

    pub fn try_get_mut(&mut self) -> Option<&mut T> {
        P::try_get_mut(&mut self.inner)
    }

    pub fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            _marker: PhantomData,
        }
    }
}

impl<T, P: Shared<T>> Deref for SharedPtr<T, P> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &*self.inner
    }
}

type RcPtr<T> = SharedPtr<T, Rc<T>>;
type ArcPtr<T> = SharedPtr<T, Arc<T>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rc_test() {
        let mut a = RcPtr::new(123);
        if let Some(x) = a.try_get_mut() {
            *x += 1;
        }
        assert_eq!(*a, 124);
    }

    #[test]
    fn arc_test() {
        let a = ArcPtr::new(123);
        let b = a.clone();
        assert_eq!(*a, *b);
    }
}

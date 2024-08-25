use core::{
    cell::UnsafeCell,
    hint::spin_loop,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicBool, Ordering},
};

use crate::riscv::registers::tp;

#[derive(Debug)]
pub struct Mutex<T> {
    locked: AtomicBool,  // Is the lock held?
    data: UnsafeCell<T>, // actual data
}

#[derive(Debug)]
pub struct MutexGuard<'a, T: 'a> {
    mutex: &'a Mutex<T>,
    who: usize,
}

impl<T> Mutex<T> {
    pub const fn new(value: T) -> Mutex<T> {
        Mutex {
            locked: AtomicBool::new(false),
            data: UnsafeCell::new(value),
        }
    }

    pub fn lock(&self) -> MutexGuard<'_, T> {
        while self
            .locked
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            spin_loop()
        }
        MutexGuard {
            who: tp::read(),
            mutex: self,
        }
    }

    pub fn unlock(guard: MutexGuard<'_, T>) -> &'_ Mutex<T> {
        guard.mutex()
    }
}

unsafe impl<T: Send> Sync for Mutex<T> {}

impl<'a, T: 'a> MutexGuard<'a, T> {
    pub fn mutex(&self) -> &'a Mutex<T> {
        self.mutex
    }

    pub fn holding(&self) -> bool {
        tp::read() == self.who
    }
}

impl<'a, T: 'a> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        assert!(
            self.holding(),
            "Hart {} tried to unlock a Mutex held by {}.",
            tp::read(),
            self.who
        );
        self.mutex.locked.store(false, Ordering::Release);
    }
}

impl<'a, T: 'a> Deref for MutexGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.mutex.data.get() }
    }
}

impl<'a, T: 'a> DerefMut for MutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.mutex.data.get() }
    }
}

unsafe impl<T: Sync> Sync for MutexGuard<'_, T> {}

use core::{
    cell::UnsafeCell,
    hint::spin_loop,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicBool, Ordering},
};

use crate::arch::riscv::registers::tp;

/// Makes sure the value it contains is only accessed in mutual exclusion.
#[derive(Debug)]
pub struct Mutex<T> {
    locked: AtomicBool,  // Is the lock held?
    data: UnsafeCell<T>, // actual data
}

/// Represents the ownership of the lock. Allows access to the value inside the mutex.
#[derive(Debug)]
pub struct MutexGuard<'a, T: 'a> {
    mutex: &'a Mutex<T>,
    who: usize,
}

impl<T> Mutex<T> {
    /// Creates a new `Mutex` containing the value `value`.
    pub const fn new(value: T) -> Mutex<T> {
        Mutex {
            locked: AtomicBool::new(false),
            data: UnsafeCell::new(value),
        }
    }

    /// Acquires the lock atomically.
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

    /// Releases the lock explicitly.
    pub fn unlock(guard: MutexGuard<'_, T>) -> &'_ Mutex<T> {
        guard.mutex()
    }
}

unsafe impl<T: Send> Sync for Mutex<T> {}

impl<'a, T: 'a> MutexGuard<'a, T> {
    /// Returns the `Mutex` this `MutexGuard` belongs to.
    pub fn mutex(&self) -> &'a Mutex<T> {
        self.mutex
    }

    /// Checks if the current `hart` is holding the lock.
    pub fn holding(&self) -> bool {
        tp::read() == self.who
    }
}

impl<'a, T: 'a> Drop for MutexGuard<'a, T> {
    /// Releases the lock implicitly.
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

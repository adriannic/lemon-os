use core::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicBool, Ordering},
};

#[derive(Debug)]
pub struct Mutex<T> {
    locked: AtomicBool,  // Is the lock held?
    data: UnsafeCell<T>, // actual data
}

#[derive(Debug)]
pub struct MutexGuard<'a, T: 'a> {
    mutex: &'a Mutex<T>,
}

impl<T> Mutex<T> {
    pub const fn new(value: T) -> Mutex<T> {
        Mutex {
            locked: AtomicBool::new(false),
            data: UnsafeCell::new(value),
        }
    }

    pub fn lock(&self) -> MutexGuard<'_, T> {
        loop {
            if self
                .locked
                .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
                .is_ok()
            {
                break MutexGuard { mutex: self };
            }
            core::hint::spin_loop()
        }
    }

    pub fn unlock(guard: MutexGuard<'_, T>) -> &'_ Mutex<T> {
        guard.mutex()
    }

    #[allow(clippy::mut_from_ref)]
    pub unsafe fn get_mut(&self) -> &mut T {
        &mut *self.data.get()
    }

    // It is only safe when used in functions such as fork_ret(),
    // where passing guards is difficult.
    pub unsafe fn force_unlock(&self) {
        self.locked.store(false, Ordering::Release);
    }
}

unsafe impl<T: Send> Sync for Mutex<T> {}

impl<'a, T: 'a> MutexGuard<'a, T> {
    // Returns a reference to the original 'Mutex' object.
    pub fn mutex(&self) -> &'a Mutex<T> {
        self.mutex
    }
}

impl<'a, T: 'a> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
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

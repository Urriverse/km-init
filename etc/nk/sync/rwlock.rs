use core::{
    cell::UnsafeCell,
    sync::atomic::{AtomicU64, Ordering},
    hint,
};

const WRITER_BIT: u64 = 1 << (u64::BITS - 1);

pub struct RwLock<T> {
    state: AtomicU64,
    data: UnsafeCell<T>,
}

unsafe impl<T: Send> Send for RwLock<T> {}
unsafe impl<T: Send> Sync for RwLock<T> {}

impl<T> RwLock<T> {
    pub const fn new(t: T) -> Self {
        Self {
            state: AtomicU64::new(0),
            data: UnsafeCell::new(t),
        }
    }

    pub fn read(&self) -> RwLockReadGuard<'_, T> {
        loop {
            let old = self.state.load(Ordering::Acquire);
            if old & WRITER_BIT != 0 {
                hint::spin_loop();
                continue;
            }
            let new = old + 1;
            debug_assert!(new & WRITER_BIT == 0);
            if self
                .state
                .compare_exchange_weak(old, new, Ordering::AcqRel, Ordering::Relaxed)
                .is_ok()
            {
                return RwLockReadGuard { lock: self };
            }
            // CAS failed, retry.
        }
    }

    pub fn try_read(&self) -> Option<RwLockReadGuard<'_, T>> {
        let old = self.state.load(Ordering::Acquire);
        if old & WRITER_BIT != 0 {
            return None;
        }
        let new = old + 1;
        if new & WRITER_BIT == 0
            && self
                .state
                .compare_exchange(old, new, Ordering::AcqRel, Ordering::Relaxed)
                .is_ok()
        {
            Some(RwLockReadGuard { lock: self })
        } else {
            None
        }
    }

    pub fn write(&self) -> RwLockWriteGuard<'_, T> {
        loop {
            let old = self.state.load(Ordering::Acquire);
            if old != 0 {
                hint::spin_loop();
                continue;
            }
            if self
                .state
                .compare_exchange_weak(0, WRITER_BIT, Ordering::AcqRel, Ordering::Relaxed)
                .is_ok()
            {
                return RwLockWriteGuard { lock: self };
            }
        }
    }

    pub fn try_write(&self) -> Option<RwLockWriteGuard<'_, T>> {
        let old = self.state.load(Ordering::Acquire);
        if old != 0 {
            return None;
        }
        if self
            .state
            .compare_exchange(0, WRITER_BIT, Ordering::AcqRel, Ordering::Relaxed)
            .is_ok()
        {
            Some(RwLockWriteGuard { lock: self })
        } else {
            None
        }
    }
}

pub struct RwLockReadGuard<'a, T> {
    lock: &'a RwLock<T>,
}

impl<T> core::ops::Deref for RwLockReadGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.lock.data.get() }
    }
}

impl<T> Drop for RwLockReadGuard<'_, T> {
    fn drop(&mut self) {
        self.lock.state.fetch_sub(1, Ordering::Release);
    }
}

pub struct RwLockWriteGuard<'a, T> {
    lock: &'a RwLock<T>,
}

impl<T> core::ops::Deref for RwLockWriteGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.lock.data.get() }
    }
}

impl<T> core::ops::DerefMut for RwLockWriteGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<T> Drop for RwLockWriteGuard<'_, T> {
    fn drop(&mut self) {
        self.lock.state.store(0, Ordering::Release);
    }
}

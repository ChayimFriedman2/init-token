use std::cell::UnsafeCell;

pub(crate) struct SyncUnsafeCell<T>(UnsafeCell<T>);

// SAFETY: This is the entire point of this type.
unsafe impl<T: Sync> Sync for SyncUnsafeCell<T> {}

impl<T> SyncUnsafeCell<T> {
    pub(crate) const fn new(value: T) -> Self {
        Self(UnsafeCell::new(value))
    }

    /// # Safety
    ///
    /// You have to ensure there are no mutable references to the inner value
    /// and will there not be as long as the returned reference is used.
    #[inline]
    pub(crate) unsafe fn get(&self) -> &T {
        // SAFETY: Precondition.
        unsafe { &*self.0.get() }
    }

    /// # Safety
    ///
    /// You have to ensure there are no references (mutable or shared) to the
    /// inner value and there will not be as long as the returned reference is used.
    #[inline]
    pub(crate) unsafe fn get_mut(&self) -> &mut T {
        // SAFETY: Precondition.
        unsafe { &mut *self.0.get() }
    }
}

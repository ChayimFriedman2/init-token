//! Helper stuff for the [`init_big!`] macro.
//!
//! [`init_big!`]: macro@crate::init_big

use std::marker::PhantomData;
use std::sync::Once;

use crate::sync_unsafe_cell::SyncUnsafeCell;
use crate::token::TokenWrapper;

/// A potentially-uninitialized `static` with all things needed to initialize it.
///
/// Call [`init()`] to gain an access token.
///
/// [`init()`]: Static::init
pub struct Static<T, Token> {
    // INVARIANT: This should only be written once, and in `init()`.
    value: SyncUnsafeCell<T>,
    init_once: Once,
    initializer: fn(&SyncUnsafeCell<T>),
    _token: PhantomData<fn(Token) -> Token>,
}

impl<T, Token: TokenWrapper> Static<T, Token> {
    /// The initializer is allowed to call `SyncUnsafeCell::get_mut()`.
    /// This is not taking `&mut T` because this gives
    /// "mutable references are not allowed in constant functions".
    #[doc(hidden)]
    pub const fn new(const_value: T, initializer: fn(&SyncUnsafeCell<T>)) -> Self {
        Self {
            value: SyncUnsafeCell::new(const_value),
            init_once: Once::new(),
            initializer,
            _token: PhantomData,
        }
    }

    /// Initialize the static if not initialized yet and gain an access token.
    ///
    /// This is little expensive to call (has to check if initialized), so if
    /// you already have an access token prefer to use it.
    #[inline]
    pub fn init(&self) -> Token {
        self.init_once.call_once(|| {
            // SAFETY: We're only touching `self.value` here and in `get_value()`.
            // Another `init()` cannot coexist with us because `call_once()` prevents that (we
            // would either return without calling the callback or block).
            // `get_value()` cannot be called yet because a precondition of it is that it is only
            // called after `init()` and we cannot enter this segment of code twice, guaranteed
            // by the `call_once()`.
            // We're not actually calling this here, see reasoning on `new()`.
            // let value = unsafe { self.value.get_mut() };

            // INVARIANT: If we already called `init()` then `call_once()` would not have called
            // us (either skipping or panicking if the initializer panicked), and there is no
            // other place we write to `self.value`. The initializer cannot store it because its
            // lifetime can be any (HRTB).
            (self.initializer)(&self.value);
        });

        // SAFETY: We initialized the static above, or panicked if the initializer panicked.
        unsafe { Token::new() }
    }

    /// # Safety
    ///
    /// `init()` must have been already called.
    #[doc(hidden)]
    #[inline]
    pub unsafe fn get_value(&self) -> &T {
        // SAFETY: We can only write to this once in `init()` (precondition), and `init()`
        // was already called so there will be no outstanding mutable references.
        unsafe { self.value.get() }
    }
}

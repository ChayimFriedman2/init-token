pub use std::fmt;
pub use std::ops::Deref;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Token {
    _private: (),
}

impl Token {
    /// # Safety
    ///
    /// Calling this function directly is always undefined behavior. Call `init()`
    /// on the static.
    ///
    /// Inside this crate, this should be called only after the static's `init()`
    /// method has been called.
    #[inline(always)]
    pub unsafe fn new() -> Self {
        Self { _private: () }
    }
}

/// # Safety
///
/// Implementing this trait is always undefined behavior. Use the macros provided by
/// this crate.
///
/// Inside this trait, `TokenWrapper::new()` must be the only way to create a token
/// instance.
pub unsafe trait TokenWrapper {
    /// # Safety
    ///
    /// Calling this function directly is always undefined behavior. Call `init()`
    /// on the static.
    ///
    /// Inside this crate, this should be called only after the static's `init()`
    /// method has been called.
    unsafe fn new() -> Self;
}

#[doc(hidden)]
#[macro_export]
macro_rules! token {
    {
        $( #[doc = $($token_doc:tt)*] )*
        $token_vis:vis token $token_name:ident
            by $static_mod:ident($static_name:ident : $static_type:ty);
    } => {
        $( #[doc = $($token_doc)*] )*
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        $token_vis struct $token_name { _token: $crate::token::Token }

        impl $crate::token::fmt::Debug for $token_name {
            fn fmt(&self, f: &mut $crate::token::fmt::Formatter<'_>) -> $crate::token::fmt::Result {
                f.debug_struct(stringify!($token_name)).finish_non_exhaustive()
            }
        }

        // SAFETY: This is the only way to instantiate the token, because it contains a field
        // of type `Token` that can only be instantiated by us (precondition of `Token::new()`)
        // and we only call it here.
        #[allow(unsafe_code)]
        unsafe impl $crate::token::TokenWrapper for $token_name {
            #[inline(always)]
            unsafe fn new() -> Self {
                // We use `unsafe` twice for `unsafe_op_in_unsafe_fn`, but those with this lint off
                // will have a "unused unsafe" warning.
                #[allow(unused_unsafe)]
                Self {
                    // SAFETY: Precondition.
                    _token: unsafe { $crate::token::Token::new() },
                }
            }
        }

        impl $token_name {
            /// This is same as the deref but provides a `'static` reference.
            ///
            /// This is an associated function and not a method in order to not collide
            /// with a method of the same name.
            #[inline(always)]
            pub fn static_ref(_this: Self) -> &'static $static_type {
                // SAFETY: The token can only be created by calling `Token::new()`
                // (precondition of `TokenWrapper`).
                // A precondition of `Token::new()` is that `init()` was called.
                unsafe { $crate::$static_mod::Static::get_value(&$static_name) }
            }
        }

        impl $crate::token::Deref for $token_name {
            type Target = $static_type;
            #[inline(always)]
            fn deref(&self) -> &Self::Target { Self::static_ref(*self) }
        }
    };
}

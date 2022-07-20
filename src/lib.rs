#![doc = include_str!("../README.md")]
#![forbid(rust_2018_idioms, unsafe_op_in_unsafe_fn)]

pub mod init;
mod sync_unsafe_cell;
#[doc(hidden)]
pub mod token;

/// A runtime-initialized `static` with zero access overhead protected by an access token.
///
/// This macro should be the default choice. Use [`init_big!`] only if the static value
/// is too big to pass on stack.
///
/// See [the crate-level documentation](crate) for details.
///
/// ```
/// init_token::init! {
///     /// The magic token to get `MY_STATIC` working.
///     pub token MyToken;
///     /// My cool static.
///     pub static MY_STATIC: i32 = "-123".parse().unwrap();
/// }
///
/// let token = MY_STATIC.init();
/// assert_eq!(*token, -123);
/// let static_ref: &'static i32 = MyToken::static_ref(token);
/// assert_eq!(*static_ref, -123);
/// // We can call `init()` twice, but then it has to check whether the `static`
/// // is initialized. Therefore, prefer using existing tokens if possible.
/// assert_eq!(*MY_STATIC.init(), -123);
/// ```
#[macro_export]
macro_rules! init {
    {
        $( #[doc = $($token_doc:tt)*] )*
        $token_vis:vis token $token_name:ident;
        $( #[doc = $($doc:tt)*] )*
        $vis:vis static $name:ident : $type:ty = $init:expr;
    } => {
        $crate::token! {
            $( #[doc = $($token_doc)*] )*
            $token_vis token $token_name by init($name : $type);
        }

        $( #[doc = $($doc)*] )*
        $vis static $name : $crate::init::Static<$type, $token_name> =
            $crate::init::Static::new(|| $init);
    };
}

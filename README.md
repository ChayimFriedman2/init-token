A crate for one-time safe initialization of static, without overhead.

There are multiple ways to initialize `static`s in Rust. The most common is [`lazy_static`](https://docs.rs/lazy_static) or [`once_cell`](https://docs.rs/once_cell) that is [even being integrated into the standard library](https://doc.rust-lang.org/nightly/std/cell/struct.LazyCell.html). Those crates lazily-initialize the value. The problem is that this incurs an overhead for each access, very small overhead but this is a problem for some applications.

This crate proposes another approach: initialization that produces a zero sized access token that you can then use to access the value via `Deref`.

There are two options on how to do that: [`init!`](https://docs.rs/init-token/latest/init_token/macro.init.html) and [`init_big!`](https://docs.rs/init-token/latest/init_token/macro.init.html). `init!` should be the default choice. Its syntax is like the following:

```rust
init_token::init! {
    /// The magic token to get `MY_STATIC` working.
    pub token MyInitToken;
    /// My cool static.
    pub static MY_STATIC: i32 = std::env::var("MY_STATIC").unwrap().parse().unwrap();
}
```

`init_big!` is intended for cases where the static value is very big, too big to pass on stack, and thus returning it from the initializer is problematic. Instead, you provide a const initializer, and then `init(name) { init_code }`, where `name` will be a mutable reference to the contents of the static. An example will explain better:

```rust
init_token::init_big! {
    /// The magic token to get `MY_STATIC` working.
    pub token MyInitToken;
    /// My cool static.
    // The initializer here must be `const` and will be put directly on the `static`'s
    // initializer expression.
    pub static MY_STATIC: i32 = 0;

    // Now, we write code to calculate the static and assign to it as we wish.
    // `my_static` here is a pointer to `MY_STATIC` and has the type `&mut i32`.
    init(my_static) {
        *my_static = std::env::var("MY_STATIC").unwrap().parse().unwrap();
    }
}
```

With `init!`, the static will be an instance of [`init::Static`](https://docs.rs/init-token/latest/init_token/init/struct.Static.html). With `init_big`, it'll be an instance of [`init_big::Static`](https://docs.rs/init-token/latest/init_token/init_big/struct.Static.html). In both cases you should call the `init()` method on the static to get an access token. The token will be some auto-generated struct, which implements `Deref` to the static's type. If you need a `'static` reference, you can instead use the `static_ref()` associated function (it is not a method - call it as `TokenType::static_ref(token)` and not as `token.static_ref()`!).

The visibilities of the `static` and the token don't have to match. This can be useful if you want to control who can get a token but let everybody use the `static` once they have a token, without carrying a reference to the static that is not zero-sized and thus has some overhead.

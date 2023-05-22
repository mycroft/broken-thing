# Why?

When trying to use the "cache" in the main scope of the function, it does not build anymore.

There is some "conflict" with the `await` when opening the file right after:

```rust
let file = tokio::fs::File::open("/tmp/testaroo").await;
```

The error:

```sh
> cargo build
   Compiling hello-rs-axum-failure v0.1.0 (/home/mycroft/dev/hello-rs-axum-failure)
error[E0277]: the trait bound `fn(Extension<Arc<std::sync::RwLock<PathBuf>>>, Request<Body>) -> impl Future<Output = Response<http_body::combinators::box_body::UnsyncBoxBody<axum::body::Bytes, axum::Error>>> {handle_proxy}: Handler<_, _, _>` is not satisfied
   --> src/main.rs:45:29
    |
45  |         .route("/:key", get(handle_proxy));
    |                         --- ^^^^^^^^^^^^ the trait `Handler<_, _, _>` is not implemented for fn item `fn(Extension<Arc<std::sync::RwLock<PathBuf>>>, Request<Body>) -> impl Future<Output = Response<http_body::combinators::box_body::UnsyncBoxBody<axum::body::Bytes, axum::Error>>> {handle_proxy}`
    |                         |
    |                         required by a bound introduced by this call
    |
    = help: the following other types implement trait `Handler<T, S, B>`:
              <Layered<L, H, T, S, B, B2> as Handler<T, S, B2>>
              <MethodRouter<S, B> as Handler<(), S, B>>
note: required by a bound in `axum::routing::get`
   --> /home/mycroft/.cargo/registry/src/index.crates.io-6f17d22bba15001f/axum-0.6.18/src/routing/method_routing.rs:403:1
    |
403 | top_level_handler_fn!(get, GET);
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `get`
    = note: this error originates in the macro `top_level_handler_fn` (in Nightly builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0277`.
error: could not compile `hello-rs-axum-failure` due to previous error
```

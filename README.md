# awaiter-trait

A `no_std` compatible Rust library providing traits for blocking on futures, enabling synchronous code to interact with asynchronous operations.

## Overview

This crate defines a hierarchy of traits that allow synchronous code to await futures:

- **`Awaiter`** / **`AwaiterMut`** - Safe traits for blocking on futures with shared (`&self`) or mutable (`&mut self`) access
- **`UnsafeAwaiter`** / **`UnsafeAwaiterMut`** - Unsafe variants for scenarios requiring manual safety guarantees
- **`Coroutine`** and related traits - Execute synchronous closures with access to an awaiter within an async context

The crate also provides:
- Dynamic trait objects (`DynAwaiter`, `DynAwaiterMut`, etc.) for type-erased awaiting
- An `autoimpl!` macro to automatically implement related traits
- Optional `embedded-io` integration for bridging async and sync I/O traits

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
awaiter-trait = "0.2"
```

### Basic Example

```rust
use awaiter_trait::{Awaiter, AwaiterMut};
use core::pin::Pin;
use core::future::Future;

// Implement an awaiter that can block on futures
struct MyAwaiter;

impl Awaiter for MyAwaiter {
    fn r#await<T>(&self, f: Pin<&mut (dyn Future<Output = T> + '_)>) -> T {
        // Implementation that blocks until the future completes
        // ...
    }
}

// Use autoimpl! to automatically implement related traits
awaiter_trait::autoimpl!(<> MyAwaiter as Awaiter);
```

### Coroutine Traits

The `Coroutine` traits allow executing synchronous code within an async context:

```rust
use awaiter_trait::Coroutine;

async fn example<C: Coroutine>(coro: &C) {
    coro.exec(|awaiter| {
        // Synchronous code that can use the awaiter
        // to block on futures
    }).await;
}
```

## Features

- **`embedded-io`** - Enables integration with `embedded-io` and `embedded-io-async` crates, providing wrappers to use async I/O types with synchronous interfaces.

## Related Crates

- **[corosensei-awaiter-trait](./corosensei-awaiter-trait)** - Provides an implementation of the `Coroutine` trait using the `corosensei` coroutine library, enabling stackful coroutines.

## License

This project is licensed under [CC0-1.0](https://creativecommons.org/publicdomain/zero/1.0/).

## Goals
- [ ] Simplify async/sync bridging for embedded and coroutine contexts
- [ ] Maintain `no_std` compatibility

## Progress
- [ ] Core `Awaiter` traits defined
- [ ] `autoimpl!` macro available
- [ ] `embedded-io` integration supported

---
*AI assisted*

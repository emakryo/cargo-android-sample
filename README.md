# Cargo configuration example for android

## Setup

### Install Rust and android toolchains

```sh
$ # Install Rust
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
$ # Install Android toolchains
$ rustup target add aarch64-linux-android
```

### Install Android NDK

See https://developer.android.com/studio/projects/install-ndk

## Build / Run / Test

```sh
$ cargo build-android
$ cargo run-android
$ cargo test-android
```

## Benchmark w/ Criterion.rs

```
$ cargo bench-android
$ open ./target/criterion_android
```

## More

https://github.com/rust-windowing/android-ndk-rs
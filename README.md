# Sample app for android with rust/cargo

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

## Benchmark

TBD

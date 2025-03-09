# jerasure-sys

Jerasure-sys is a Rust FFI binding for [`gf-complete`](https://github.com/ceph/gf-complete) and [`jerasure`](https://github.com/tsuraan/Jerasure), which provide erasure coding in C. This project allows you to use Galois computation and erasure codes functionality in your Rust applications.

## Requirements

This crate bundles the source code of `gf-complete` and `jerasure`, and compiles these libraries during the build process, so you don't need to have these two libraries pre-installed on your system.

However, this crate uses the [`bindgen`](https://crates.io/crates/bindgen) and [`autotools`](https://crates.io/crates/autotools) crates, which depend on the following packages:
- libllvm
- libtool
- autoconf
- automake

On Ubuntu-like distributions, you can install these dependencies using:
``` shell
apt install autoconf automake libtool libclang-dev
```

## Contributing

Feel free to open an issue. If you've got a fix or feature ready, open a PR. Thanks!

## License

MIT
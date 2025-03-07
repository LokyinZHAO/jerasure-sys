# jerasure-sys

Jerasure-sys is a Rust FFI binding for [`gf-complete`](https://github.com/ceph/gf-complete) and [`jerasure`](https://github.com/tsuraan/Jerasure), which provides erasure coding in C. This project allows you to use Galois computation and erasure codes functionality in your Rust applications.

## Requirements

This crate bundles the source code of `gf-complete` and `jerasure`, and compiles these libs during the build process, so you don't need to have any libraries pre-installed on your system.

## Contributing

Feel free to open an issue. If you've got a fix or feature ready, open a PR. Thanks!

## License

MIT
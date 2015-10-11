# Rust Bindings for libespeak

Following convention of crates suffixed with `-sys`, the crate is no more than a bottom-level wrapper for the API
exposed by `libespeak`'s header.

## Dependencies
`espeak-sys` requires that you have `libespeak` somewhere on your computer where Rust's linker may find it.
To ensure `libespeak` is installed if you have a Debian-based Linux system, simply run the command

```
sudo apt-get install libespeak-dev
```

in your terminal.
`libespeak` may not be available, or may not be usable with this crate on platforms other than Linux.

## Usage
Just add `espeak-sys` as a dependency in `Cargo.toml` and link to it within your crate:

```toml
[dependencies]
espeak-sys = "0.0.2"
```

```rust
extern crate espeak_sys;
```

Usage of the crate is as simple as using the provided functions as they are defined in `libespeak`'s header file, with a few caveats.
Namely, the `type` field of the `espeak_EVENT` struct is renamed to `event_type`, as the former is a reserved keyword in Rust.
Additionally, due to the lack of a one-to-one correspondence for C's `union` types, the `id` field is represented by a mere `u64`
that the crate's user is expected to mercilessly transmute to the desired type variant.
Since `id`'s type is dependent on the value of `event_type`, it should still be possible to build a safe wrapper upon the struct.

### Finding Help
The function of each of the symbols this crate exposes is best documented in the C header file:

* [espeak API Documentation](http://espeak.sourceforge.net/speak_lib.h)

## License
Copyright © 2015 Chandler Atchley

Distributed under the [GNU GPL v3 License](LICENSE).


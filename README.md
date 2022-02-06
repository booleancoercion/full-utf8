# utf8-rfc2279

An implementation of UTF-8 encoding, according to the obsolete [RFC2279].  
The main difference between `utf8-rfc2279` and an implementation of [RFC3629] is that `utf8-rfc2279` does not check for valid byte sequences and has a length limit of 6 bytes instead of 4.

### **Do not use this crate as a regular UTF-8 encoder without additional checks! Rust's standard library handles this for you.**

## `no_std`

`utf8-rfc2279` is `no_std` and has no allocations, as it does not need them.

## License

Licensed under either of:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

[RFC2279]: https://datatracker.ietf.org/doc/html/rfc2279
[RFC3629]: https://datatracker.ietf.org/doc/html/rfc3629
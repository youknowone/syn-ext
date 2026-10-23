# syn-ext: shortcuts & editable interface

## Deprecated

This crate is deprecated. Use [syn](https://docs.rs/syn/) directly instead.

syn-ext is a shortcut utility over syn. With LLM assistance available for writing
the equivalent code directly against syn, this shortcut abstraction is no longer
worth maintaining as a separate crate. Prefer direct syn APIs and keep any small,
project-specific helpers in your own code.

```rust
use syn_ext::ext::*;
```

- Documentation: [https://docs.rs/syn-ext/](https://docs.rs/syn-ext/)
- Crates: [https://crates.io/crates/syn-ext](https://crates.io/crates/syn-ext)
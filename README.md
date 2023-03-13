# optional-error

[![crates.io](https://img.shields.io/crates/v/optional-error?style=flat-square)](https://crates.io/crates/optional-error)
[![docs.rs](https://img.shields.io/docsrs/optional-error?style=flat-square)](https://docs.rs/optional-error)

This crate provides a simpler way to create and manage an `Option<syn::Error>`.

```rust
fn parse(input: DeriveInput) -> Result<TokenStream, syn::Error> {
    // Create an optional error to contain zero or more errors
    let mut errors = OptionalError::default();

    if !matches!(input.vis, Visibility::Public(_)) {
        // Combine with a new error (or initialize if empty)
        errors.combine(syn::Error::new(Span::call_site(), "input must be marked `pub`"));
    }

    match input.data {
        syn::Data::Struct(_) | syn::Data::Enum(_) => { /* ... */ }
        syn::Data::Union(_) => {
            // Combine some more!
            errors.combine(syn::Error::new(Span::call_site(), "unions not supported"));
        }
    }

    // Easy early return with all errors (if any)
    errors.try_throw()?;

    // ...
}
```
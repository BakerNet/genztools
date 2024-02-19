# GenZTools

[![Crates.io Version](https://img.shields.io/crates/v/genztools)](https://crates.io/crates/genztools)
[![docs.rs](https://img.shields.io/docsrs/genztools)](https://docs.rs/genztools/latest/genztools/)

Making Rust more accessible and readable for the upcoming genration of Rust engineers

## Examples:

Before:

```rust
let is_logged_in = get_user().await.ok().flatten().is_some();
```

After:

```rust
let is_logged_in = get_user().await.bet().on_a_stack().no_cap();
```

or

Before:

```rust
let new_thing = my_opt.map(|x| transform(x)).unwrap_or_default();
```

After:

```rust
let new_thing = my_opt.glow_up(|x| transform(x)).on_god_or_basic();
```

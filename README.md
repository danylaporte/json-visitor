# json-visitor

This crate make it easy to modify serde_json::Value.

## Example

```rust
use json_visitor::{Key, Parent, visit_mut};
use serde_json::Value;

let json = r#"[{"hello":"m"},{"foo":"bar"}]"#;

let mut value = serde_json::from_str(&json).unwrap();

// transform `m` value from `hello` into `world`
visit_mut(&mut value, |value, parent| {
    if parent.key == Key::Prop("hello") {
        *value = Value::String("world".into());
    }

    true    // true to continue deeping..
});

let json = serde_json::to_string(&value).unwrap();

assert_eq!(json, r#"[{"hello":"world"},{"foo":"bar"}]"#);
```

## License

This project is licensed under the terms of the MIT license.

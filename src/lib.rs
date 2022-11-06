use serde_json::Value;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Key<'a> {
    Index(usize),
    Prop(&'a str),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Parent<'a> {
    pub key: Key<'a>,
    pub parent: Option<&'a Parent<'a>>,
}

/// Visit nodes in a `Value` and mutates them using a mutator fn
///
/// # Example
///
/// ```
/// use json_visitor::{Key, Parent, visit_mut};
/// use serde_json::Value;
///
/// let json = r#"[{"hello":"m"},{"foo":"bar"}]"#;
///
/// let mut value = serde_json::from_str(&json).unwrap();
///
/// visit_mut(&mut value, |value, parent| {
///     if parent.key == Key::Prop("hello") {
///         *value = Value::String("world".into());
///     }
///
///     true
/// });
///
/// let json = serde_json::to_string(&value).unwrap();
/// assert_eq!(json, r#"[{"hello":"world"},{"foo":"bar"}]"#);
/// ```
pub fn visit_mut<Mutator>(value: &mut Value, mut mutator: Mutator)
where
    Mutator: FnMut(&mut Value, &Parent) -> bool,
{
    visit_mut_sub(value, &mut mutator, None)
}

fn visit_mut_impl<'a, Mutator>(value: &mut Value, mutator: &mut Mutator, parent: Parent<'a>)
where
    Mutator: FnMut(&mut Value, &Parent) -> bool,
{
    if mutator(value, &parent) {
        visit_mut_sub(value, mutator, Some(&parent));
    }
}

fn visit_mut_sub<Mutator>(value: &mut Value, mutator: &mut Mutator, parent: Option<&Parent>)
where
    Mutator: FnMut(&mut Value, &Parent) -> bool,
{
    match value {
        Value::Array(a) => a.iter_mut().enumerate().for_each(|(index, item)| {
            visit_mut_impl(
                item,
                mutator,
                Parent {
                    key: Key::Index(index),
                    parent,
                },
            )
        }),

        Value::Object(o) => o.iter_mut().for_each(|(k, v)| {
            visit_mut_impl(
                v,
                mutator,
                Parent {
                    key: Key::Prop(k),
                    parent,
                },
            )
        }),

        _ => {}
    }
}

# Code Style

## Fluent setters

Use `derive_setters::Setters` to generate fluent setters that only assign a field and return `Self`.

Use opt-out generation:

```rust
#[derive(derive_setters::Setters, IntoElement)]
pub struct Component {
    disabled: bool,
    #[setters(skip)]
    base: Div,
}
```

### Generated setters

Generate a setter when the method would only perform one of these assignments:

```rust
self.field = value;
self.field = value.into();
self.field = Some(value);
```

Use field options where required:

- `#[setters(into)]` for `impl Into<T>` parameters.
- `#[setters(strip_option)]` when the public setter accepts `T` and stores `Some(T)`.
- Combine them as `#[setters(into, strip_option)]` when both apply.

Do not apply `strip_option` automatically. `Option<T>` and nested options may represent meaningful controlled/uncontrolled states.

### Skipped fields

Every field that is not a plain public component property must explicitly use `#[setters(skip)]`. This includes:

- GPUI base elements;
- child collections;
- runtime, context, registration, and wiring state;
- callbacks and handlers;
- style closures;
- resolver, serializer, comparator, and formatter functions;
- fields requiring validation, normalization, wrapping, or multi-field updates.

Keep the corresponding method handwritten when it pushes or extends a collection, wraps a closure in `Rc`, converts an element, updates multiple fields, or otherwise expresses component behavior.

```rust
#[derive(derive_setters::Setters, IntoElement)]
pub struct Component {
    disabled: bool,

    #[setters(into, strip_option)]
    name: Option<SharedString>,

    #[setters(skip)]
    children: Vec<ComponentChild>,

    #[setters(skip)]
    on_change: Option<ComponentChangeHandler>,
}
```

When adding a field to a struct deriving `Setters`, decide explicitly whether it is a public Base UI property. If it is not, mark it with `#[setters(skip)]` in the same change.

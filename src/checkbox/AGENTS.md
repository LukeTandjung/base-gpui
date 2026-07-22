# Checkbox Implementation Notes

This file captures Checkbox-specific implementation details for `crates/base_gpui/src/checkbox`.
Generic component architecture belongs in `docs/base-gpui-component-architecture.md`; keep this file focused on Checkbox behavior and local contracts.

## Component family

The Checkbox port exposes these explicit component names:

- `CheckboxRoot`
- `CheckboxIndicator`

## Public props and API surface

`CheckboxRoot` owns root-level Checkbox props:

- `.checked(...)`
- `.default_checked(...)`
- `.on_checked_change(...)`
- `.indeterminate(...)`
- `.disabled(...)`
- `.read_only(...)`
- `.required(...)`
- form-related public props (`name`, `value`, `form`, `parent`, `unchecked_value`)

`CheckboxIndicator` owns only indicator-local props:

- `.keep_mounted(...)`

Do not move root props onto the indicator.

## Controlled checked semantics

`CheckboxRoot` uses `Option<bool>` for the controlled checked prop:

- `None` = uncontrolled prop absent
- `Some(value)` = controlled checked value

Behavior:

- Controlled mode: caller passes `.checked(...)`; interactions call `on_checked_change(...)` but do not mutate the source of truth.
- Uncontrolled mode: caller omits `.checked(...)`; interactions call `on_checked_change(...)` and mutate runtime checked state.
- `default_checked(...)` initializes uncontrolled checked state.
- `disabled` and `read_only` prevent toggling and prevent callbacks.
- `indeterminate` affects style state and indicator presence, but activating an indeterminate checkbox does not clear `indeterminate` by itself.

## File layout

Checkbox uses the runtime/context split from the component architecture:

```text
crates/base_gpui/src/checkbox/
  actions.rs
  child.rs            # CheckboxChild typed routing
  child_wiring.rs     # private child context attachment
  context.rs          # CheckboxContext: entity plumbing + controlled/uncontrolled rule
  props.rs            # CheckboxProps and callback type
  style_state.rs      # root and indicator style-state structs
  runtime.rs          # CheckboxRuntime: checked value, focus state, commands, queries
  layers/
    checkbox_indicator.rs
    checkbox_root.rs
```

There is no `CheckboxState`, no `GenericContext`, and no shared generic child abstraction in Checkbox. The checked value lives in `CheckboxRuntime`.

## Context, props, runtime

`CheckboxContext` owns exactly the injection/plumbing facts:

```rust
runtime: Entity<CheckboxRuntime>
props: Rc<CheckboxProps>
controlled: Rc<Option<Option<bool>>>
```

It exposes three method shapes:

- `read(...)`
- `update(...)`
- `toggle(...)`

Do not grow Checkbox rendering logic on `CheckboxContext`. Checkbox behavior belongs on `CheckboxRuntime`; `CheckboxContext::toggle(...)` only resolves controlled/uncontrolled state and fires `on_checked_change` after the runtime update returns its outcome.

`CheckboxProps` owns stable root props and callbacks. It must not own runtime state.

`CheckboxRuntime` owns Checkbox-specific runtime facts:

- checked value,
- focused state.

## Typed child routing

`CheckboxRoot` keeps typed `CheckboxChild` children before GPUI erases to `AnyElement`.

`CheckboxChild` currently routes:

- `CheckboxIndicator`

Typed child-routing enums belong in `checkbox/child.rs`, not `checkbox/layers/`. Private context attachment belongs in `checkbox/child_wiring.rs`, not a shared generic child abstraction.

## Activation and focus

Checkbox uses GPUI actions/key dispatch, not raw key-down handlers.

`checkbox/actions.rs` defines:

- `CheckboxToggle`

`CHECKBOX_ROOT_KEY_CONTEXT` scopes Space activation to the Checkbox root.

Activation behavior:

- Mouse click toggles when enabled and not read-only.
- Space toggles when focused, enabled, and not read-only.
- Enter does not toggle.
- Disabled and read-only checkboxes do not toggle and do not call `on_checked_change`.

Focus behavior:

- `CheckboxRoot` owns the stable keyed `FocusHandle`.
- Root render syncs `focus_handle.is_focused(window)` into `CheckboxRuntime`.
- Focus state is exposed through `CheckboxRootStyleState`.

## Style-state styling

Checkbox exposes state-aware styling through `style_with_state(...)` on:

- `CheckboxRoot`
- `CheckboxIndicator`

Style-state structs:

- `CheckboxRootStyleState`: checked, unchecked, disabled, read-only, required, indeterminate, and focused.
- `CheckboxIndicatorStyleState`: root style state and indicator presence.

Indicator presence is true when:

- `keep_mounted = true`, or
- root is checked, or
- root is indeterminate.

Do not add DOM-style data attributes unless they become useful GPUI API.

## Accessibility (AccessKit)

`CheckboxRoot` is the only accessible part:

- `.role(Role::CheckBox)` on the root element chain.
- `.aria_toggled(...)` from `CheckboxRootStyleState`: `Toggled::Mixed` when `indeterminate`, else `Toggled::True` / `Toggled::False` from `checked`.
- `.aria_label(...)` builder on `CheckboxRoot` (literal string; gpui has no `aria-labelledby` id wiring).
- `Action::Click` and `Action::Focus` are auto-registered by the existing `.on_click(...)` / `.track_focus(...)`; AT-dispatched Click is synthesized by gpui as left mouse down/up at the node center, so it arrives as `ClickEvent::Mouse` and passes the root's mouse-only guard into `request_checkbox_toggle`. No explicit `on_a11y_action` handler is needed.

`CheckboxIndicator` is presentational: no role, kept out of the a11y tree, matching Base UI.

Double-announce rule: when a caller renders a visible text label inside `CheckboxRoot` **and** sets `.aria_label(...)`, the visible text should use `Text::new_inaccessible(...)` instead of `text!(...)` so the label is not announced twice.

Known gaps (no gpui builder in the pinned revision; omitted, pending upstream):

- `aria-disabled` / native `disabled` announcement — runtime already blocks toggles and disabled roots are removed from tab order, but AT will not announce "disabled".
- `aria-readonly` — runtime blocks toggles; no announcement.
- `aria-required` — kept in `CheckboxRootStyleState` only.
- `aria-labelledby` — use the literal `.aria_label(...)` instead.

## Base UI differences / intentionally dropped web details

Do not port:

- DOM ARIA attributes (accessibility goes through GPUI AccessKit builders; see above),
- DOM form submission behavior until explicitly implemented,
- React hooks/context details,
- SSR/hydration/prehydration scripts,
- DOM data attributes.

Translate behavior into GPUI-native runtime, focus handles, style state, and actions.

<a id="v0.2.0"></a>
# [v0.2.0](https://github.com/rhysd/keybinds-rs/releases/tag/v0.2.0) - 2025-05-26

- **BREAKING:** Upgrade dependencies
  - bitflags v2.9.0
  - crossterm v0.29.0 ([#1](https://github.com/rhysd/keybinds-rs/issues/1))
- Fix warnings in `minimal` and `vim` examples

[Changes][v0.2.0]


<a id="v0.1.1"></a>
# [v0.1.1](https://github.com/rhysd/keybinds-rs/releases/tag/v0.1.1) - 2025-03-21

- Handle keypad keys and application keys from termwiz's key events.
- Add `Key::ZoomToggle` key.
- Fix `ZoomIn` and `ZoomOut` keys are not converted while handling iced and winit key events.

[Changes][v0.1.1]


<a id="v0.1.0"></a>
# [v0.1.0](https://github.com/rhysd/keybinds-rs/releases/tag/v0.1.0) - 2025-03-16

The first stable release :tada:

- **BREAKING:** Remove `Key::Backtab` because it is actually not mapped to a single key. crossterm's `BackTab` key is now correctly mapped to <kbd>Shift</kbd> + <kbd>Tab</kbd>.
- Define the [minimum supported Rust version](https://github.com/rhysd/keybinds-rs?tab=readme-ov-file#minimal-supported-rust-version-msrv) is **1.80.0**.
- Clarify the [versioning of this crate](https://github.com/rhysd/keybinds-rs?tab=readme-ov-file#versioning) in documents.
- Add the [contribution guide](https://github.com/rhysd/keybinds-rs/blob/main/CONTRIBUTING.md).
- Add the [`vim` example](https://github.com/rhysd/keybinds-rs/blob/main/examples/vim.rs) to demonstrate how to use this crate for modal key bindings.
- Add `Keybinds::ongoing_inputs` to get the key inputs matching to some key bindings.
- Add `Key::Help` variant.
- Fix parsing `NumLock` key.

[Changes][v0.1.0]


<a id="v0.0.9"></a>
# [v0.0.9](https://github.com/rhysd/keybinds-rs/releases/tag/v0.0.9) - 2025-03-09

- **BREAKING:** Merge `KeybindDispatcher` into `Keybinds`. Now `Keybinds` directly has `dispatch` method and dispatches actions. This change makes the API simpler. Note that setting different timeout values to compared objects makes `==` operator return `false` even if the key bindings are completely equal.
- **BREAKING:** Rewrite `KeySeq` using [smallvec](https://docs.rs/smallvec/latest/smallvec). Now it is a struct instead of enum and it no longer allows accessing the internal state.
  - Key sequences containing up to 2 key inputs can be allocated on stack instead of heap (previously only a single key input could be allocated on stack).
  - `KeySeq` now implements `Hash` trait.
  - Methods for adding inputs to a key sequence were implemented.
- **BREAKING:** Replace `Key::F(u8)` variant with `Key::F1`...`Key::F35` variants. This change brings the following benefits.
  - The size of `Key` was reduced from 8 bytes to 4 bytes because `u8` value required a 3 bytes padding.
  - Invalid keys such as `Key::F(0)` or `Key::F(99)` are now not possible.
- Implement `Extend` for `Keybinds` to append multiple key bindings.
- Fix crossterm's button release event is not ignored as `Key::Ignored`.
- Fix an empty key sequence does not cause an error on serialization.
- Explain the crate features in [Cargo.toml](https://github.com/rhysd/keybinds-rs/blob/main/Cargo.toml).

[Changes][v0.0.9]


<a id="v0.0.8"></a>
# [v0.0.8](https://github.com/rhysd/keybinds-rs/releases/tag/v0.0.8) - 2025-03-06

- **BREAKING:** `KeyInput::new` now takes `Into<Mods>` instead of `Mods`.
- **BREAKING:** Implement `FromIterator<Keybind>` for `Keybinds` instead of `KeybindDispatcher`.
- Add API documents to all the APIs. Read it on [docs.rs](https://docs.rs/keybinds/latest/keybinds/).
- Add `KeybindDispatcher::into_keybinds`.
- Add `Keybinds::take_dispatcher` method.
- Add the support for F21~F35 keys by iced and winit.


[Changes][v0.0.8]


<a id="v0.0.7"></a>
# [v0.0.7](https://github.com/rhysd/keybinds-rs/releases/tag/v0.0.7) - 2025-02-25

- Support [iced](https://crates.io/crates/iced) as optional `iced` feature. ([example](https://github.com/rhysd/keybinds-rs/blob/main/examples/iced.rs))
- Fix Shift modifier is not converted correctly from platform-specific events.
- Fix Meta modifier key is wrongly handled as Cmd key on converting crossterm events.
- Scrape examples on generating API documents for docs.rs.
- Add the [fuzz tests](https://github.com/rhysd/keybinds-rs/tree/main/fuzz).
- Refine the [crossterm example](https://github.com/rhysd/keybinds-rs/blob/main/examples/crossterm.rs) with more practical usage.

[Changes][v0.0.7]


<a id="v0.0.6"></a>
# [v0.0.6](https://github.com/rhysd/keybinds-rs/releases/tag/v0.0.6) - 2025-02-22

- Support serializing key bindings with [serde](https://crates.io/crates/serde) crate. `serde` feature needs to be enabled. ([example](https://github.com/rhysd/keybinds-rs/blob/main/examples/serialize.rs))
- Support structural fuzzing with [arbitrary](https://crates.io/crates/arbitrary) crate. `arbitrary` feature needs to be enabled. ([example](https://github.com/rhysd/keybinds-rs/blob/main/examples/arbitrary.rs))
- Implement `Display` trait for `Key`, `Mod`, `KeyInput`, and `KeySeq`.
- Improve an error message on Shift modifier with unnamed keys.

[Changes][v0.0.6]


<a id="v0.0.5"></a>
# [v0.0.5](https://github.com/rhysd/keybinds-rs/releases/tag/v0.0.5) - 2025-02-22

- **BREAKING:** Make some `new` constructor functions into `From::from` trait implementations with more generic arguments using `Into` trait.
- **BREAKING:** Avoid heap allocation on a single-key key binding by changing `KeySeq` struct to an enum.
- **BREAKING:** Rename `KeySeq::matches` to `KeySeq::match_to`.
- **BREAKING:** Hide `KeyInput`'s fields and add getter methods `KeyInput::key`, `KeyInput::mods`.
- Add support for `Shift` modifier again. `Shift` modifier key is only available with named keys like `Shift+Up`. For example, when you want to define key binding for <kbd>Shift</kbd> + <kbd>A</kbd>, you need to use `A` instead of `Shift+a`. This restriction is for avoiding some confusing edge cases and may be relaxed in the future.
- Make fields of `Keybind` struct public.
- Fix error handling on parsing invalid empty key bindings.
- Fix non-ASCII spaces like `U+3000` are not available for key binding.
- Define the key binding syntax in the separate [document](https://github.com/rhysd/keybinds-rs/blob/main/doc/binding_syntax.md).

[Changes][v0.0.5]


<a id="v0.0.4"></a>
# [v0.0.4](https://github.com/rhysd/keybinds-rs/releases/tag/v0.0.4) - 2025-02-19

- **BREAKING:** Use logical keys instead of physical keys. Now the characters in key sequence are case-sensitive. For example, logical key `A` means typing "A" and "Shift" physical keys and it is distinguished from `a` (only typing "A"). Along with this change, `Shift` modifier key was removed.
- **BREAKING:** Rename `KeybindDispatcher::trigger` to `KeybindDispatcher::dispatch`.
- Implement `FromIterator<Keybind<A>>` for `KeybindDispatcher<A>` to constructor a dispatcher instance from a list of key bindings easily.
- Implement `Deref<Target=[KeyBind]>` for `Keybinds` and implement `KeybindDispatcher::keybinds` getter method.
- Add API document for all optional features.
- Add `minimal` and `serde` examples.

[Changes][v0.0.4]


<a id="v0.0.3"></a>
# [v0.0.3](https://github.com/rhysd/keybinds-rs/releases/tag/v0.0.3) - 2025-02-17

- **BREAKING:** Key bindings API was renamed.
  - `KeyBindMatcher` → `KeybindDispatcher`
  - `KeyBinds` → `Keybinds`
  - `KeyBind` → `Keybind`
- **BREAKING:** Some methods were renamed.
  - `KeybindDispatcher::timeout` → `KeybindDispatcher::set_timeout`
- Support [termwiz](https://crates.io/crates/termwiz) as optional `termwiz` feature.
- Allow modifiers in upper case like `CTRL`.
- Implement `KeybindDispatcher::bind` and `KeybindDispatcher::add` methods for easily defining key bindings.

[Changes][v0.0.3]


<a id="v0.0.2"></a>
# [v0.0.2](https://github.com/rhysd/keybinds-rs/releases/tag/v0.0.2) - 2025-02-15

- Support [winit](https://crates.io/crates/winit) as optional `winit` feature.
- Add `Key::Ignored` special key to ignore on checking key bindings. This is used when converting events which are actually not related to key inputs.
- Fix key release is not ignored.
- Fix modifier-only keys are converted to `Key::Unidentified`.
- Fix converting crossterm key event with shift modifier pressed.

[Changes][v0.0.2]


<a id="v0.0.1"></a>
# [v0.0.1](https://github.com/rhysd/keybinds-rs/releases/tag/v0.0.1) - 2025-02-13

- Separate support for parsing key bindings with serde as `serde` feature
- Make all features opt-in
- Remove anyhow crate dependency
- Support [crossterm](https://github.com/crossterm-rs/crossterm) as optional `crossterm` feature
- Fix the ongoing key sequence is not properly cleared on timeout

[Changes][v0.0.1]


<a id="v0.0.0"></a>
# [v0.0.0](https://github.com/rhysd/keybinds-rs/releases/tag/v0.0.0) - 2025-02-13

The first pre-release with incomplete minimal implementation. Note that the development is still ongoing and many bugs and API changes will be expected until v0.1.0 first stable release.

[Changes][v0.0.0]


[v0.2.0]: https://github.com/rhysd/keybinds-rs/compare/v0.1.1...v0.2.0
[v0.1.1]: https://github.com/rhysd/keybinds-rs/compare/v0.1.0...v0.1.1
[v0.1.0]: https://github.com/rhysd/keybinds-rs/compare/v0.0.9...v0.1.0
[v0.0.9]: https://github.com/rhysd/keybinds-rs/compare/v0.0.8...v0.0.9
[v0.0.8]: https://github.com/rhysd/keybinds-rs/compare/v0.0.7...v0.0.8
[v0.0.7]: https://github.com/rhysd/keybinds-rs/compare/v0.0.6...v0.0.7
[v0.0.6]: https://github.com/rhysd/keybinds-rs/compare/v0.0.5...v0.0.6
[v0.0.5]: https://github.com/rhysd/keybinds-rs/compare/v0.0.4...v0.0.5
[v0.0.4]: https://github.com/rhysd/keybinds-rs/compare/v0.0.3...v0.0.4
[v0.0.3]: https://github.com/rhysd/keybinds-rs/compare/v0.0.2...v0.0.3
[v0.0.2]: https://github.com/rhysd/keybinds-rs/compare/v0.0.1...v0.0.2
[v0.0.1]: https://github.com/rhysd/keybinds-rs/compare/v0.0.0...v0.0.1
[v0.0.0]: https://github.com/rhysd/keybinds-rs/tree/v0.0.0

<!-- Generated by https://github.com/rhysd/changelog-from-release v3.9.0 -->

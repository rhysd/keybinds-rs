use crate::{Key, KeyInput, KeySeq, Match, Result};
use std::mem;
use std::ops::Deref;
use std::time::{Duration, Instant};

#[cfg(feature = "arbitrary")]
use arbitrary::Arbitrary;

/// Single key binding. A pair of key sequence and its action.
///
/// ```
/// use keybinds::{KeybindDispatcher, Keybind, KeySeq, KeyInput, Key, Mods};
///
/// struct Action;
///
/// let mut dispatcher = KeybindDispatcher::default();
/// dispatcher.push(Keybind::new('x', Action));
/// dispatcher.push(Keybind::new(KeyInput::new(Key::Left, Mods::CTRL), Action));
/// dispatcher.push(Keybind::new(KeySeq::from(vec!['H'.into(), 'i'.into()]), Action));
/// ```
#[derive(Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
pub struct Keybind<A> {
    /// The key sequence that triggers the action.
    pub seq: KeySeq,
    /// The action triggered by the key sequence.
    pub action: A,
}

impl<A> Keybind<A> {
    /// Create a new key binding.
    ///
    /// ```
    /// use keybinds::{Keybind, KeySeq, KeyInput, Key, Mods};
    ///
    /// struct Action;
    ///
    /// // Single-key key bindings
    /// let _ = Keybind::new('x', Action);
    /// let _ = Keybind::new(Key::Enter, Action);
    /// let _ = Keybind::new(KeyInput::new('x', Mods::CTRL), Action);
    ///
    /// // Complex key binding ("Ctrl+Up Alt+Down")
    /// let _ = Keybind::new(
    ///     KeySeq::from(vec![
    ///         KeyInput::new(Key::Up, Mods::CTRL),
    ///         KeyInput::new(Key::Down, Mods::ALT),
    ///     ]),
    ///     Action,
    /// );
    /// ```
    pub fn new<S: Into<KeySeq>>(seq: S, action: A) -> Self {
        Self {
            seq: seq.into(),
            action,
        }
    }
}

/// The result of finding a matched key binding by [`Keybinds::find`].
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Found<'a, A> {
    /// The keybind corresponding to the key inputs was found.
    Keybind(&'a Keybind<A>),
    /// No keybind completely matched to the key inputs but some keybinds may match in the future. That means the
    /// matching is still ongoing.
    Ongoing,
    /// No keybind matched.
    None,
}

/// An array of key bindings used by [`KeybindDispatcher`].
///
/// This implements [`Deref`] for dereferencing as `&[Keybind]` so it allows accessing the elements by the immutable
/// methods. Note that this struct does not implement `DerefMut` to prevent the dispatcher state from being broken by
/// modifying the key bindings.
///
/// ```
/// use keybinds::{Keybinds, Keybind, Key, KeybindDispatcher};
///
/// enum Action {
///     Hello,
///     Goodbye,
/// }
///
/// let keybinds = Keybinds::from(vec![
///     Keybind::new('H', Action::Hello),
///     Keybind::new(Key::Enter, Action::Goodbye),
///     // ...
/// ]);
/// let dispatcher = KeybindDispatcher::new(keybinds);
/// ```
#[derive(Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
pub struct Keybinds<A>(Vec<Keybind<A>>);

impl<A> Keybinds<A> {
    /// Search a key binding which is matched to the given inputs.
    ///
    /// The search result is matched or ongoing or unmatched. Please read the [`Found`] document for more details. When
    /// the inputs match to multiple key bindings, the first one found will be returned.
    ///
    /// ```
    /// use keybinds::{Keybinds, Keybind, KeySeq, Found};
    ///
    /// #[derive(Debug, PartialEq)]
    /// enum Action {
    ///     Foo,
    ///     Bar,
    /// }
    ///
    /// let binds = Keybinds::from(vec![
    ///     Keybind::new(KeySeq::from(vec!['a'.into(), 'b'.into()]), Action::Foo),
    ///     Keybind::new(KeySeq::from(vec!['a'.into(), 'c'.into()]), Action::Bar),
    /// ]);
    ///
    /// // When 'a' is input, key bindings "a b" and "a c" are ongoing.
    /// assert_eq!(binds.find(&['a'.into()]), Found::Ongoing);
    ///
    /// // Inputs 'a' → 'b' matches to the key binding "a b".
    /// assert!(
    ///     matches!(
    ///         binds.find(&['a'.into(), 'b'.into()]),
    ///         Found::Keybind(matched) if matched.action == Action::Foo,
    ///     ),
    /// );
    ///
    /// // Input 'x' matches nothing.
    /// assert_eq!(binds.find(&['x'.into()]), Found::None);
    /// ```
    pub fn find(&self, inputs: &[KeyInput]) -> Found<'_, A> {
        let mut saw_prefix = false;
        for bind in self.0.iter() {
            match bind.seq.match_to(inputs) {
                Match::Matched => return Found::Keybind(bind),
                Match::Prefix => saw_prefix = true,
                Match::Unmatch => continue,
            }
        }
        if saw_prefix {
            Found::Ongoing
        } else {
            Found::None
        }
    }

    /// Create a [`KeybindDispatcher`] instance from the key bindings.
    ///
    /// This method is a utility to easily create a dispatcher instance from your configuration without moving out
    /// the [`Keybinds`] instance. After calling this method, this instance is cleared to avoid a heap allocation.
    ///
    /// ```
    /// use keybinds::{Keybinds, Keybind};
    ///
    /// struct Action;
    ///
    /// // Configuration of your app
    /// struct Config {
    ///     bindings: Keybinds<Action>,
    /// }
    ///
    /// // Get your app configuration from somewhere (e.g. parsing from a configuration file)
    /// let mut config = Config {
    ///     bindings: Keybinds::from(vec![
    ///         Keybind::new('a', Action),
    ///         // ...
    ///     ]),
    /// };
    ///
    /// // Create a dispatcher instance without moving out `bindings` field from the `Config` instance
    /// let mut dispatcher = config.bindings.take_dispatcher();
    ///
    /// # fn do_something(_config: &Config) {}
    /// // The `Config` instance is still accessible here
    /// do_something(&config);
    /// ```
    pub fn take_dispatcher(&mut self) -> KeybindDispatcher<A> {
        KeybindDispatcher::new(mem::take(self))
    }
}

/// Create an empty keybinds instance.
///
/// ```
/// use keybinds::Keybinds;
///
/// struct Action;
///
/// assert!(Keybinds::<Action>::default().is_empty());
/// ```
impl<A> Default for Keybinds<A> {
    fn default() -> Self {
        Self(vec![])
    }
}

impl<A> Deref for Keybinds<A> {
    type Target = [Keybind<A>];

    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

impl<A> From<Vec<Keybind<A>>> for Keybinds<A> {
    fn from(binds: Vec<Keybind<A>>) -> Self {
        Self(binds)
    }
}

/// Convert [`Keybinds`] into an array of [`Keybind`] instances. This method is useful to update the key bindings.
///
/// ```
/// use keybinds::{Keybinds, Keybind};
///
/// enum Action {
///     Foo,
///     Bar,
/// }
///
/// let binds = Keybinds::from(vec![Keybind::new('a', Action::Foo)]);
///
/// // Update the bindings later
/// let mut binds: Vec<_> = binds.into();
/// binds.push(Keybind::new('b', Action::Bar));
///
/// // Recreate the `Keybinds` instance again.
/// let binds: Keybinds<_> = binds.into();
/// ```
impl<A> From<Keybinds<A>> for Vec<Keybind<A>> {
    fn from(binds: Keybinds<A>) -> Self {
        binds.0
    }
}

/// The default timeout value of the key binding matching by [`KeybindDispatcher`].
///
/// The interval of key inputs must be smaller than it. The default value is 1 second.
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(1);

/// A dispatcher that takes key inputs and dispatches the corresponding key bindings' actions.
///
/// The [`KeybindDispatcher::dispatch`] method dispatches an action for the given key input. The dispatcher checks
/// a key sequence. When the sequence matches to one of the defined key bindings, it returns the corresponding action.
/// Note that it does not wait for an additional input if some key binding matches to the sequence. For example,
/// if "a b" and "a b c" are defined, the sequence "a" → "b" matches to the binding "a b" and "a b c" will never be
/// triggered.
///
/// If the interval of key inputs exceed the timeout (default to 1 second), the key sequence breaks there. For example,
/// when "b" input follows "a" input after 2 seconds, each inputs "a" and "b" are treated as single key inputs, not a
/// key sequence "a b". Please see [`KeybindDispatcher::set_timeout`] for the code example.
///
/// ```
/// use keybinds::{KeybindDispatcher, KeyInput, Key, Mods};
///
/// #[derive(PartialEq, Eq, Debug)]
/// enum Action {
///     Foo,
///     Bar,
/// }
///
/// let mut dispatcher = KeybindDispatcher::default();
///
/// // Key sequence "f" → "o" → "o"
/// dispatcher.bind("f o o", Action::Foo).unwrap();
/// // Sequence of key combinations
/// dispatcher.bind("Ctrl+b Ctrl+a", Action::Bar).unwrap();
///
/// assert_eq!(dispatcher.dispatch('f'), None);
/// assert_eq!(dispatcher.dispatch('o'), None);
/// assert_eq!(dispatcher.dispatch('o'), Some(&Action::Foo));
///
/// assert_eq!(dispatcher.dispatch(KeyInput::new('b', Mods::CTRL)), None);
/// assert_eq!(dispatcher.dispatch(KeyInput::new('a', Mods::CTRL)), Some(&Action::Bar));
/// ```
pub struct KeybindDispatcher<A> {
    binds: Keybinds<A>,
    ongoing: Vec<KeyInput>,
    last_input: Option<Instant>,
    timeout: Duration,
}

impl<A> Default for KeybindDispatcher<A> {
    /// Create an empty dispatcher.
    ///
    /// ```
    /// use keybinds::KeybindDispatcher;
    ///
    /// struct Action;
    ///
    /// let mut dispatcher = KeybindDispatcher::default();
    /// assert!(dispatcher.keybinds().is_empty());
    ///
    /// dispatcher.bind("Ctrl+X", Action).unwrap();
    /// assert!(!dispatcher.keybinds().is_empty());
    /// ```
    fn default() -> Self {
        Self::new(Keybinds::default())
    }
}

impl<A> KeybindDispatcher<A> {
    /// Create a dispatcher instance from the given key bindings. This constructor function is useful for creating
    /// a dispatcher from [`Keybinds`] or `Vec<Keybind>`. If you want to create a dispatcher from
    /// `Iterator<Item = Keybind>`, [`KeybindDispatcher::collect`] is also useful.
    ///
    /// ```
    /// use keybinds::{Keybind, KeybindDispatcher, Key, Mods, KeyInput};
    ///
    /// enum Action {
    ///     Foo,
    ///     Bar,
    ///     Piyo,
    /// }
    ///
    /// let binds = vec![
    ///     Keybind::new('a', Action::Foo),
    ///     Keybind::new(Key::Enter, Action::Bar),
    ///     Keybind::new(KeyInput::new(Key::Up, Mods::CTRL), Action::Piyo),
    /// ];
    ///
    /// let dispatcher = KeybindDispatcher::new(binds);
    /// assert_eq!(dispatcher.keybinds().len(), 3);
    /// ```
    pub fn new<K: Into<Keybinds<A>>>(binds: K) -> Self {
        Self {
            binds: binds.into(),
            ongoing: vec![],
            last_input: None,
            timeout: DEFAULT_TIMEOUT,
        }
    }

    /// Push a new [`Keybind`] instance. If this method is called while some maching is ongoing, the matching is reset.
    ///
    /// ```
    /// use keybinds::{KeybindDispatcher, Keybind};
    ///
    /// struct Action;
    ///
    /// let mut dispatcher = KeybindDispatcher::default();
    ///
    /// dispatcher.push(Keybind::new('x', Action));
    /// assert_eq!(dispatcher.keybinds().len(), 1);
    /// ```
    pub fn push(&mut self, bind: Keybind<A>) {
        self.binds.0.push(bind);
        self.reset();
    }

    /// Define a new key binding. If the key sequence does not follow the [syntax](https://github.com/rhysd/keybinds-rs/blob/main/doc/binding_syntax.md),
    /// this method returns an error.
    ///
    /// ```
    /// use keybinds::{KeybindDispatcher, Keybind, KeyInput, Mods};
    ///
    /// #[derive(PartialEq, Eq, Debug)]
    /// struct Action;
    ///
    /// let mut dispatcher = KeybindDispatcher::default();
    ///
    /// dispatcher.bind("Ctrl+x Ctrl+y", Action).unwrap();
    /// dispatcher.bind("Foo+x", Action).unwrap_err(); // Unknown modifier "Foo"
    ///
    /// assert_eq!(dispatcher.keybinds().len(), 1);
    ///
    /// // Dispatch the action
    /// assert_eq!(dispatcher.dispatch(KeyInput::new('x', Mods::CTRL)), None);          // Matching is ongoing
    /// assert_eq!(dispatcher.dispatch(KeyInput::new('y', Mods::CTRL)), Some(&Action)); // Dispatched
    /// ```
    pub fn bind(&mut self, key_sequence: &str, action: A) -> Result<()> {
        let seq: KeySeq = key_sequence.parse()?;
        self.push(Keybind::new(seq, action));
        Ok(())
    }

    fn handle_timeout(&mut self) {
        let now = Instant::now();
        let is_timeout = self
            .last_input
            .is_some_and(|t| now.duration_since(t) > self.timeout);
        if is_timeout {
            self.ongoing.clear();
        }
        self.last_input = Some(now);
    }

    /// Dispatch an action for the given key input.
    ///
    /// This method accepts various values which implement `Into<KeyInput>`. For example, `char` value is converted
    /// into a single-character key input with no modifiers. Conversions from key event types in several frameworks
    /// are supported by enabling the optional features.
    ///
    /// ```
    /// use keybinds::{KeybindDispatcher, KeyInput, Key, Mods};
    ///
    /// #[derive(PartialEq, Eq, Debug)]
    /// enum Action {
    ///     Foo,
    /// }
    ///
    /// let mut dispatcher = KeybindDispatcher::default();
    ///
    /// dispatcher.bind("f Ctrl+o Enter", Action::Foo).unwrap();
    ///
    /// // Input "f" key with no modifiers
    /// assert_eq!(dispatcher.dispatch('f'), None);
    /// // Input "o" key with Ctrl modifier
    /// assert_eq!(dispatcher.dispatch(KeyInput::new('o', Mods::CTRL)), None);
    /// // Input "Enter" key with no modifiers
    /// assert_eq!(dispatcher.dispatch(Key::Enter), Some(&Action::Foo));
    /// ```
    pub fn dispatch<I: Into<KeyInput>>(&mut self, input: I) -> Option<&A> {
        let input = input.into();
        if input.key() == Key::Ignored {
            return None;
        }
        self.handle_timeout();
        self.ongoing.push(input);

        // `self.reset` cannot be called because the borrow checker needs to split field lifetimes.
        match self.binds.find(&self.ongoing) {
            Found::Keybind(bind) => {
                self.ongoing.clear();
                self.last_input = None;
                Some(&bind.action)
            }
            Found::Ongoing => None, // Matching is still ongoing
            Found::None => {
                self.ongoing.clear();
                self.last_input = None;
                None
            }
        }
    }

    /// Set the timeout to wait for the next key input while matching to key bindings is ongoing. For the default
    /// timeout value, see [`DEFAULT_TIMEOUT`].
    ///
    /// ```
    /// use std::time::Duration;
    /// use std::thread::sleep;
    /// use keybinds::KeybindDispatcher;
    ///
    /// struct Action;
    ///
    /// let mut dispatcher = KeybindDispatcher::default();
    /// dispatcher.bind("a b", Action).unwrap();
    ///
    /// // Set the timeout to very small value to demonstrate the usage.
    /// dispatcher.set_timeout(Duration::from_millis(10));
    ///
    /// // Input the first key input of key sequence "a b"
    /// assert!(dispatcher.dispatch('a').is_none());
    ///
    /// // Make the ongoing match expire (50ms > 10ms)
    /// sleep(Duration::from_millis(50));
    ///
    /// // Input the second key input of key sequence "a b". However it does not dispatch the action
    /// // because the matching expired.
    /// assert!(dispatcher.dispatch('b').is_none());
    /// ```
    pub fn set_timeout(&mut self, timeout: Duration) {
        self.timeout = timeout;
    }

    /// Reset the state of the dispatcher. This resets the ongoing matching of key binding.
    ///
    /// ```
    /// use keybinds::KeybindDispatcher;
    ///
    /// struct Action;
    ///
    /// let mut dispatcher = KeybindDispatcher::default();
    /// dispatcher.bind("a b", Action).unwrap();
    ///
    /// assert!(dispatcher.dispatch('a').is_none());
    ///
    /// // Abandon the ongoing matching for "a b"
    /// dispatcher.reset();
    ///
    /// assert!(dispatcher.dispatch('b').is_none());
    /// ```
    pub fn reset(&mut self) {
        self.ongoing.clear();
        self.last_input = None;
    }

    /// Get the timeout of key binding matching. See [`KeybindDispatcher::set_timeout`] to know the details of the
    /// timeout.
    ///
    /// ```
    /// use std::time::Duration;
    /// use keybinds::{KeybindDispatcher, DEFAULT_TIMEOUT};
    ///
    /// struct Action;
    ///
    /// let mut dispatcher = KeybindDispatcher::<Action>::default();
    /// assert_eq!(dispatcher.timeout(), DEFAULT_TIMEOUT);
    ///
    /// let duration = Duration::from_millis(500);
    /// dispatcher.set_timeout(duration);
    /// assert_eq!(dispatcher.timeout(), duration);
    /// ```
    pub fn timeout(&self) -> Duration {
        self.timeout
    }

    /// Get the reference to the inner [`Keybinds`] instance.
    ///
    /// ```
    /// use keybinds::{KeybindDispatcher, Keybinds, Keybind};
    ///
    /// #[derive(Clone, PartialEq, Eq, Debug)]
    /// struct Action;
    ///
    /// let binds = Keybinds::from(vec![Keybind::new('a', Action)]);
    ///
    /// let dispatcher = KeybindDispatcher::new(binds.clone());
    /// assert_eq!(dispatcher.keybinds(), &binds);
    /// ```
    pub fn keybinds(&self) -> &Keybinds<A> {
        &self.binds
    }

    /// Return whether the matching for key bindings is ongoing.
    ///
    /// ```
    /// use keybinds::KeybindDispatcher;
    ///
    /// struct Action;
    ///
    /// let mut dispatcher = KeybindDispatcher::default();
    /// dispatcher.bind("a b", Action).unwrap();
    ///
    /// assert!(!dispatcher.is_ongoing());
    /// dispatcher.dispatch('a');
    /// assert!(dispatcher.is_ongoing());
    /// dispatcher.dispatch('b');
    /// assert!(!dispatcher.is_ongoing());
    /// ```
    pub fn is_ongoing(&self) -> bool {
        self.last_input.is_some()
    }

    // TODO: Add `into_keybinds`
}

// TODO: Move this to `FromIterator<Keybind<A>> for Keybinds<A>`
impl<A> FromIterator<Keybind<A>> for KeybindDispatcher<A> {
    fn from_iter<T: IntoIterator<Item = Keybind<A>>>(iter: T) -> Self {
        Self::new(Keybinds(iter.into_iter().collect()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Key, Mods};
    use std::thread::sleep;

    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    enum A {
        Action1,
        Action2,
        Action3,
        Action4,
    }

    #[test]
    fn handle_input() {
        let binds = vec![
            Keybind::new('a', A::Action1),
            Keybind::new(KeyInput::new('a', Mods::CTRL), A::Action2),
            Keybind::new(vec!['B'.into(), 'c'.into()], A::Action3),
            Keybind::new(Key::Up, A::Action4),
        ];

        let mut keybinds = KeybindDispatcher::new(Keybinds(binds.clone()));

        for bind in binds {
            keybinds.reset();
            let len = bind.seq.as_slice().len();
            for (idx, input) in bind.seq.as_slice().iter().enumerate() {
                let is_last = idx + 1 == len;
                let expected = is_last.then_some(bind.action);
                let actual = keybinds.dispatch(input.clone());
                assert_eq!(actual, expected.as_ref(), "bind={bind:?}");
                assert_eq!(keybinds.is_ongoing(), !is_last, "bind={bind:?}");
            }
        }
    }

    #[test]
    fn discard_ongoing_nothing_matched() {
        let binds = vec![Keybind::new('a', A::Action1)];
        let mut keybinds = KeybindDispatcher::new(binds);

        assert_eq!(keybinds.dispatch('x'), None);
        assert_eq!(keybinds.dispatch('y'), None);
        assert_eq!(keybinds.dispatch('a'), Some(&A::Action1));
        assert_eq!(keybinds.dispatch('z'), None);
        assert_eq!(keybinds.dispatch('a'), Some(&A::Action1));
    }

    #[test]
    fn dispatcher_from_iter() {
        let expected = vec![
            Keybind::new('a', A::Action1),
            Keybind::new(
                vec![
                    KeyInput::new('b', Mods::CTRL),
                    KeyInput::new('c', Mods::MOD),
                ],
                A::Action2,
            ),
        ];

        let actual: KeybindDispatcher<_> = expected.clone().into_iter().collect();
        assert_eq!(*actual.binds, expected);
    }

    #[test]
    fn dispatcher_is_ongoing() {
        let binds = vec![Keybind::new(vec!['a'.into(), 'b'.into()], A::Action1)];
        let mut dispatcher = KeybindDispatcher::new(binds);

        assert!(!dispatcher.is_ongoing());
        dispatcher.dispatch('x');
        assert!(!dispatcher.is_ongoing());
        dispatcher.dispatch('a');
        assert!(dispatcher.is_ongoing());
        dispatcher.dispatch('b');
        assert!(!dispatcher.is_ongoing());
        dispatcher.dispatch('y');
        assert!(!dispatcher.is_ongoing());
        dispatcher.dispatch('a');
        assert!(dispatcher.is_ongoing());
        dispatcher.dispatch('z');
        assert!(!dispatcher.is_ongoing());
    }

    #[test]
    fn dispatcher_set_timeout() {
        let mut dispatcher = KeybindDispatcher::<A>::default();
        assert_eq!(dispatcher.timeout(), DEFAULT_TIMEOUT);
        let d = Duration::from_secs(2);
        dispatcher.set_timeout(d);
        assert_eq!(dispatcher.timeout(), d);
    }

    #[test]
    fn dispatcher_ignore_keys() {
        let binds = vec![Keybind::new(vec!['a'.into(), 'b'.into()], A::Action1)];
        let mut dispatcher = KeybindDispatcher::new(binds);
        dispatcher.dispatch('a');
        assert_eq!(dispatcher.dispatch(Key::Ignored), None);
        assert_eq!(dispatcher.dispatch('b'), Some(&A::Action1));
    }

    #[test]
    fn dispatcher_timeout_input() {
        let binds = vec![Keybind::new(vec!['a'.into(), 'b'.into()], A::Action1)];
        let mut dispatcher = KeybindDispatcher::new(binds);
        dispatcher.set_timeout(Duration::from_millis(10));

        dispatcher.dispatch('a');
        assert_eq!(dispatcher.dispatch('b'), Some(&A::Action1));

        dispatcher.dispatch('a');
        sleep(Duration::from_millis(50));
        assert_eq!(dispatcher.dispatch('b'), None);

        dispatcher.dispatch('a');
        assert_eq!(dispatcher.dispatch('b'), Some(&A::Action1));
    }

    #[test]
    fn dispatcher_bind() {
        let mut dispatcher = KeybindDispatcher::default();

        dispatcher.bind("x", A::Action1).unwrap();
        dispatcher.bind("a b", A::Action2).unwrap();
        dispatcher.bind("", A::Action1).unwrap_err();

        assert_eq!(dispatcher.dispatch('x'), Some(&A::Action1));
        dispatcher.dispatch('a');
        assert_eq!(dispatcher.dispatch('b'), Some(&A::Action2));

        dispatcher.dispatch('a');
        assert!(dispatcher.is_ongoing());
        dispatcher.bind("y", A::Action1).unwrap();
        assert!(!dispatcher.is_ongoing());
    }

    #[test]
    fn dispatcher_reset() {
        let binds = vec![Keybind::new(vec!['a'.into(), 'b'.into()], A::Action1)];
        let mut dispatcher = KeybindDispatcher::new(binds);
        dispatcher.dispatch('a');
        assert!(dispatcher.is_ongoing());
        dispatcher.reset();
        assert!(!dispatcher.is_ongoing());
    }

    #[test]
    fn default_dispatcher() {
        let mut dispatcher = KeybindDispatcher::<()>::default();
        assert!(dispatcher.keybinds().is_empty());
        assert_eq!(dispatcher.dispatch('a'), None);
        assert!(!dispatcher.is_ongoing());
    }

    #[test]
    fn distinguish_bindings_with_modifiers() {
        let binds = vec![
            Keybind::new(KeyInput::new('a', Mods::CTRL | Mods::ALT), A::Action1),
            Keybind::new(KeyInput::new('a', Mods::CTRL), A::Action2),
            Keybind::new('a', A::Action3),
        ];
        let mut dispatcher = KeybindDispatcher::new(binds);

        assert_eq!(dispatcher.dispatch('a'), Some(&A::Action3));
        assert_eq!(
            dispatcher.dispatch(KeyInput::new('a', Mods::CTRL)),
            Some(&A::Action2),
        );
        assert_eq!(
            dispatcher.dispatch(KeyInput::new('a', Mods::CTRL | Mods::ALT)),
            Some(&A::Action1),
        );
        assert_eq!(
            dispatcher.dispatch(KeyInput::new('a', Mods::CTRL | Mods::ALT | Mods::WIN)),
            None,
        );
    }

    #[test]
    fn keybinds_priority_order() {
        let binds = vec![
            Keybind::new('a', A::Action1),
            Keybind::new('a', A::Action2),
            Keybind::new('a', A::Action3),
        ];
        let mut dispatcher = KeybindDispatcher::new(binds);
        assert_eq!(dispatcher.dispatch('a'), Some(&A::Action1));
    }

    #[test]
    fn smaller_seq_is_prioritized() {
        let binds = vec![
            Keybind::new('a', A::Action1),
            Keybind::new(vec!['a'.into(), 'a'.into()], A::Action2),
            Keybind::new(vec!['a'.into(), 'b'.into()], A::Action3),
        ];
        let mut dispatcher = KeybindDispatcher::new(binds);

        assert_eq!(dispatcher.dispatch('a'), Some(&A::Action1));
        assert_eq!(dispatcher.dispatch('a'), Some(&A::Action1));
        assert_eq!(dispatcher.dispatch('b'), None);
    }

    #[test]
    fn non_ascii_space() {
        let binds = vec![Keybind::new('　', A::Action1)];
        let mut dispatcher = KeybindDispatcher::new(binds);
        assert_eq!(dispatcher.dispatch('　'), Some(&A::Action1));

        let mut dispatcher = KeybindDispatcher::default();
        dispatcher.bind("　", A::Action1).unwrap();
        dispatcher.bind("Ctrl+　", A::Action2).unwrap();
        assert_eq!(dispatcher.dispatch('　'), Some(&A::Action1));
        assert_eq!(
            dispatcher.dispatch(KeyInput::new('　', Mods::CTRL)),
            Some(&A::Action2),
        );
    }

    #[test]
    fn keybinds_take_dispatcher() {
        let mut binds = Keybinds::from(vec![
            Keybind::new('a', A::Action1),
            Keybind::new('b', A::Action2),
        ]);
        let mut dispatcher = binds.take_dispatcher();
        assert!(binds.is_empty());
        assert_eq!(dispatcher.keybinds().len(), 2);
        assert_eq!(dispatcher.dispatch('a'), Some(&A::Action1));
    }
}

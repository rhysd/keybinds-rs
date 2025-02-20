use crate::{KeyInput, KeySeq, Match, Result};
use std::ops::Deref;
use std::time::{Duration, Instant};

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Keybind<A> {
    pub seq: KeySeq,
    pub action: A,
}

impl<A> Keybind<A> {
    pub fn new<S: Into<KeySeq>>(seq: S, action: A) -> Self {
        Self {
            seq: seq.into(),
            action,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Found<'a, A> {
    Keybind(&'a Keybind<A>),
    Ongoing,
    None,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Keybinds<A>(Vec<Keybind<A>>);

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

impl<A> Keybinds<A> {
    pub fn find(&self, seq: &[KeyInput]) -> Found<'_, A> {
        let mut saw_prefix = false;
        for bind in self.0.iter() {
            match bind.seq.matches(seq) {
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
}

impl<A> From<Vec<Keybind<A>>> for Keybinds<A> {
    fn from(binds: Vec<Keybind<A>>) -> Self {
        Self(binds)
    }
}

pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(1);

pub struct KeybindDispatcher<A> {
    binds: Keybinds<A>,
    ongoing: Vec<KeyInput>,
    last_input: Option<Instant>,
    timeout: Duration,
}

impl<A> Default for KeybindDispatcher<A> {
    fn default() -> Self {
        Self::new(Keybinds::default())
    }
}

impl<A> KeybindDispatcher<A> {
    pub fn new<K: Into<Keybinds<A>>>(binds: K) -> Self {
        Self {
            binds: binds.into(),
            ongoing: vec![],
            last_input: None,
            timeout: DEFAULT_TIMEOUT,
        }
    }

    pub fn add<K: Into<KeySeq>>(&mut self, key: K, action: A) {
        self.binds.0.push(Keybind::new(key, action));
    }

    pub fn bind(&mut self, key: &str, action: A) -> Result<()> {
        let seq: KeySeq = key.parse()?;
        self.add(seq, action);
        Ok(())
    }

    pub fn set_timeout(&mut self, timeout: Duration) {
        self.timeout = timeout;
    }

    pub fn reset(&mut self) {
        self.ongoing.clear();
        self.last_input = None;
    }

    pub fn timeout(&self) -> Duration {
        self.timeout
    }

    pub fn keybinds(&self) -> &Keybinds<A> {
        &self.binds
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

    pub fn dispatch<I: Into<KeyInput>>(&mut self, input: I) -> Option<&A> {
        let input = input.into();
        if input.is_ignored() {
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
}

impl<A> FromIterator<Keybind<A>> for KeybindDispatcher<A> {
    fn from_iter<T: IntoIterator<Item = Keybind<A>>>(iter: T) -> Self {
        Self::new(Keybinds(iter.into_iter().collect()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Key, Mods};

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
            Keybind::new(
                vec![
                    KeyInput::new('B', Mods::NONE),
                    KeyInput::new('c', Mods::NONE),
                ],
                A::Action3,
            ),
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
            }
        }
    }

    #[test]
    fn discard_ongoing_nothing_matched() {
        let binds = vec![Keybind::new('a', A::Action1)];
        let mut keybinds = KeybindDispatcher::new(Keybinds(binds.clone()));

        assert_eq!(keybinds.dispatch(KeyInput::from('x')), None);
        assert_eq!(keybinds.dispatch(KeyInput::from('y')), None);
        assert_eq!(keybinds.dispatch(KeyInput::from('a')), Some(&A::Action1));
        assert_eq!(keybinds.dispatch(KeyInput::from('z')), None);
        assert_eq!(keybinds.dispatch(KeyInput::from('a')), Some(&A::Action1));
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
        assert_eq!(actual.binds.0, expected);
    }
}

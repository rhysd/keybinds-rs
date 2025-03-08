//! Support for [`arbitrary`] crate.
//!
//! This module provides [`Arbitrary`] trait support for several types in keybinds crate.
//!
//! ```
//! use arbitrary::{Arbitrary, Result, Unstructured};
//! use keybinds::{Key, KeyInput, KeySeq, Keybind, Keybinds, Mods};
//!
//! // Actions dispatched by key bindings.
//! #[derive(Arbitrary, Debug)]
//! enum Action {
//!     Hello,
//!     Goodbye,
//! }
//!
//! let raw_data = b"
//!     Hello, the document for arbitrary crate support!
//!     This is the random data input from fuzzer.
//! ";
//! let mut unstructured = Unstructured::new(raw_data);
//!
//! // Generate arbitrary instances of types in keybinds crate
//! let _ = Key::arbitrary(&mut unstructured).unwrap();
//! let _ = Mods::arbitrary(&mut unstructured).unwrap();
//! let _ = KeyInput::arbitrary(&mut unstructured).unwrap();
//! let _ = KeySeq::arbitrary(&mut unstructured).unwrap();
//! let _ = Keybind::<Action>::arbitrary(&mut unstructured).unwrap();
//! let _ = Keybinds::<Action>::arbitrary(&mut unstructured).unwrap();
//! ```
use crate::{Keybinds, Mods};
use arbitrary::{Arbitrary, Result, Unstructured};

// Note: We don't use bitflags crate's `arbitrary` feature because it is quite inefficient.
// Almost all bit patterns are generated by `Unstructured` are incorrect and they cause
// 'Incorrect format' error which means generating an arbitrary instance failed.
impl Arbitrary<'_> for Mods {
    fn arbitrary(u: &mut Unstructured<'_>) -> Result<Self> {
        let mut mods = Self::NONE;
        if u.arbitrary()? {
            mods |= Mods::CTRL;
        }
        if u.arbitrary()? {
            mods |= Mods::CMD;
        }
        if u.arbitrary()? {
            mods |= Mods::ALT;
        }
        if u.arbitrary()? {
            mods |= Mods::WIN;
        }
        if u.arbitrary()? {
            mods |= Mods::SHIFT;
        }
        Ok(mods)
    }
}

// Note: Do not generate abitrary values for timeout and ongoing key sequence.
impl<'a, A: Arbitrary<'a>> Arbitrary<'a> for Keybinds<A> {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        Ok(Self::new(u.arbitrary()?))
    }
}

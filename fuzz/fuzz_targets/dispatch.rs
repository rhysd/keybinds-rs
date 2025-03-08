#![no_main]
use arbitrary::{Arbitrary, Result, Unstructured};
use keybinds::{KeyInput, Keybinds};
use libfuzzer_sys::fuzz_target;

fn run(data: &[u8]) -> Result<()> {
    let mut u = Unstructured::new(data);
    let mut dispatcher = Keybinds::<u8>::new(u.arbitrary()?);
    for _ in 0..20 {
        let _ = dispatcher.dispatch(KeyInput::arbitrary(&mut u)?);
    }
    Ok(())
}

fuzz_target!(|data: &[u8]| {
    let _ = run(data);
});

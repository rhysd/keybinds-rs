#![no_main]
use arbitrary::{Arbitrary, Result, Unstructured};
use keybinds::{KeyInput, KeybindDispatcher, Keybinds};
use libfuzzer_sys::fuzz_target;

fn run(data: &[u8]) -> Result<()> {
    let mut u = Unstructured::new(data);
    let binds: Keybinds<u8> = u.arbitrary()?;
    let mut dispatcher = KeybindDispatcher::new(binds);
    for _ in 0..20 {
        let _ = dispatcher.dispatch(KeyInput::arbitrary(&mut u)?);
    }
    Ok(())
}

fuzz_target!(|data: &[u8]| {
    let _ = run(data);
});

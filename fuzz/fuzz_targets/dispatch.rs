#![no_main]
use arbitrary::{Arbitrary, Result, Unstructured};
use keybinds::{KeyInput, Keybinds, KeybindsOld};
use libfuzzer_sys::fuzz_target;

fn run(data: &[u8]) -> Result<()> {
    let mut u = Unstructured::new(data);
    let binds: KeybindsOld<u8> = u.arbitrary()?;
    let mut dispatcher = Keybinds::new(binds);
    for _ in 0..20 {
        let _ = dispatcher.dispatch(KeyInput::arbitrary(&mut u)?);
    }
    Ok(())
}

fuzz_target!(|data: &[u8]| {
    let _ = run(data);
});

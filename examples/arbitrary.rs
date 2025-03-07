use arbitrary::{Arbitrary, Result, Unstructured};
use keybinds::{Key, KeyInput, KeySeq, Keybind, Keybinds, Mods};

// Actions dispatched by key bindings.
#[derive(Arbitrary, Debug)]
enum Action {
    Hello,
    Goodbye,
}

fn main() -> Result<()> {
    let raw_data = b"
        Hello, an example for arbitrary crate support!
        This is the random data input from fuzzer.
    ";
    let mut unstructured = Unstructured::new(raw_data);

    // Generate arbitrary instances of types in keybinds crate
    println!("{:?}", Key::arbitrary(&mut unstructured)?);
    println!("{:?}", Mods::arbitrary(&mut unstructured)?);
    println!("{:?}", KeyInput::arbitrary(&mut unstructured)?);
    println!("{:?}", KeySeq::arbitrary(&mut unstructured)?);
    println!("{:?}", Keybind::<Action>::arbitrary(&mut unstructured)?);
    println!("{:?}", Keybinds::<Action>::arbitrary(&mut unstructured)?);

    Ok(())
}

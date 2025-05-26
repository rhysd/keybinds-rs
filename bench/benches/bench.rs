use criterion::{criterion_group, criterion_main, Criterion};
use keybinds::{Key, KeyInput, KeySeq, Keybinds, Mods};
use std::hint::black_box;
use std::str::FromStr;

#[derive(Clone)]
struct Action;

fn parse(c: &mut Criterion) {
    c.bench_function("parse::single_char", |b| {
        b.iter(|| black_box(KeySeq::from_str("a").unwrap()))
    });
    c.bench_function("parse::short_multiple_chars", |b| {
        b.iter(|| black_box(KeySeq::from_str("h i").unwrap()))
    });
    c.bench_function("parse::long_multiple_chars", |b| {
        b.iter(|| black_box(KeySeq::from_str("h e l l o , w o r l d !").unwrap()))
    });
    c.bench_function("parse::single_char_with_mod", |b| {
        b.iter(|| black_box(KeySeq::from_str("Ctrl+A").unwrap()))
    });
    c.bench_function("parse::multiple_chars_with_mod", |b| {
        b.iter(|| black_box(KeySeq::from_str("Ctrl+H Cmd+E Mod+L Alt+L Super+O").unwrap()))
    });
    c.bench_function("parse::single_named_char", |b| {
        b.iter(|| black_box(KeySeq::from_str("Tab").unwrap()))
    });
    c.bench_function("parse::short_multiple_named_chars", |b| {
        b.iter(|| black_box(KeySeq::from_str("Tab Enter").unwrap()))
    });
    c.bench_function("parse::long_multiple_named_chars", |b| {
        b.iter(|| black_box(KeySeq::from_str("Up Up Down Down Left Right Left Right B A").unwrap()))
    });
}

fn dispatch(c: &mut Criterion) {
    let mut keybinds = Keybinds::default();
    keybinds.bind("a", Action).unwrap();
    keybinds.bind("Down", Action).unwrap();
    keybinds.bind("Ctrl+Alt+a", Action).unwrap();
    keybinds.bind("b x", Action).unwrap();
    keybinds.bind("Left Right", Action).unwrap();
    keybinds.bind("Ctrl+b Ctrl+x", Action).unwrap();
    keybinds.bind("h e l l o , w o r l d !", Action).unwrap();
    keybinds
        .bind(
            "Up Up Down Down Left Right Left Right Backspace Enter",
            Action,
        )
        .unwrap();
    keybinds
        .bind(
            "Ctrl+h Ctrl+e Ctrl+l Ctrl+l Ctrl+o Ctrl+, Ctrl+w Ctrl+o Ctrl+r Ctrl+l Ctrl+d Ctrl+!",
            Action,
        )
        .unwrap();

    c.bench_function("dispatch::single_unnamed", |b| {
        let mut keybinds = keybinds.clone();
        b.iter(|| {
            assert!(keybinds.dispatch('a').is_some());
        })
    });
    c.bench_function("dispatch::single_named", |b| {
        let mut keybinds = keybinds.clone();
        b.iter(|| {
            assert!(keybinds.dispatch(Key::Down).is_some());
        })
    });
    c.bench_function("dispatch::single_mods", |b| {
        let mut keybinds = keybinds.clone();
        b.iter(|| {
            assert!(keybinds
                .dispatch(KeyInput::new('a', Mods::CTRL | Mods::ALT))
                .is_some());
        })
    });
    c.bench_function("dispatch::short_seq_unnamed", |b| {
        let mut keybinds = keybinds.clone();
        b.iter(|| {
            assert!(keybinds.dispatch('b').is_none());
            assert!(keybinds.dispatch('x').is_some());
        })
    });
    c.bench_function("dispatch::short_seq_named", |b| {
        let mut keybinds = keybinds.clone();
        b.iter(|| {
            assert!(keybinds.dispatch(Key::Left).is_none());
            assert!(keybinds.dispatch(Key::Right).is_some());
        })
    });
    c.bench_function("dispatch::short_seq_mods", |b| {
        let mut keybinds = keybinds.clone();
        b.iter(|| {
            assert!(keybinds.dispatch(KeyInput::new('b', Mods::CTRL)).is_none());
            assert!(keybinds.dispatch(KeyInput::new('x', Mods::CTRL)).is_some());
        })
    });
    c.bench_function("dispatch::long_seq_unnamed", |b| {
        let mut keybinds = keybinds.clone();
        b.iter(|| {
            assert!(keybinds.dispatch('h').is_none());
            assert!(keybinds.dispatch('e').is_none());
            assert!(keybinds.dispatch('l').is_none());
            assert!(keybinds.dispatch('l').is_none());
            assert!(keybinds.dispatch('o').is_none());
            assert!(keybinds.dispatch(',').is_none());
            assert!(keybinds.dispatch('w').is_none());
            assert!(keybinds.dispatch('o').is_none());
            assert!(keybinds.dispatch('r').is_none());
            assert!(keybinds.dispatch('l').is_none());
            assert!(keybinds.dispatch('d').is_none());
            assert!(keybinds.dispatch('!').is_some());
        })
    });
    c.bench_function("dispatch::long_seq_named", |b| {
        let mut keybinds = keybinds.clone();
        b.iter(|| {
            assert!(keybinds.dispatch(Key::Up).is_none());
            assert!(keybinds.dispatch(Key::Up).is_none());
            assert!(keybinds.dispatch(Key::Down).is_none());
            assert!(keybinds.dispatch(Key::Down).is_none());
            assert!(keybinds.dispatch(Key::Left).is_none());
            assert!(keybinds.dispatch(Key::Right).is_none());
            assert!(keybinds.dispatch(Key::Left).is_none());
            assert!(keybinds.dispatch(Key::Right).is_none());
            assert!(keybinds.dispatch(Key::Backspace).is_none());
            assert!(keybinds.dispatch(Key::Enter).is_some());
        })
    });
    c.bench_function("dispatch::long_seq_mods", |b| {
        let mut keybinds = keybinds.clone();
        b.iter(|| {
            assert!(keybinds.dispatch(KeyInput::new('h', Mods::CTRL)).is_none());
            assert!(keybinds.dispatch(KeyInput::new('e', Mods::CTRL)).is_none());
            assert!(keybinds.dispatch(KeyInput::new('l', Mods::CTRL)).is_none());
            assert!(keybinds.dispatch(KeyInput::new('l', Mods::CTRL)).is_none());
            assert!(keybinds.dispatch(KeyInput::new('o', Mods::CTRL)).is_none());
            assert!(keybinds.dispatch(KeyInput::new(',', Mods::CTRL)).is_none());
            assert!(keybinds.dispatch(KeyInput::new('w', Mods::CTRL)).is_none());
            assert!(keybinds.dispatch(KeyInput::new('o', Mods::CTRL)).is_none());
            assert!(keybinds.dispatch(KeyInput::new('r', Mods::CTRL)).is_none());
            assert!(keybinds.dispatch(KeyInput::new('l', Mods::CTRL)).is_none());
            assert!(keybinds.dispatch(KeyInput::new('d', Mods::CTRL)).is_none());
            assert!(keybinds.dispatch(KeyInput::new('!', Mods::CTRL)).is_some());
        })
    });
    c.bench_function("dispatch::unmatch_unnamed", |b| {
        let mut keybinds = keybinds.clone();
        b.iter(|| {
            assert!(keybinds.dispatch('w').is_none());
        })
    });
    c.bench_function("dispatch::unmatch_named", |b| {
        let mut keybinds = keybinds.clone();
        b.iter(|| {
            assert!(keybinds.dispatch(Key::Tab).is_none());
        })
    });
    c.bench_function("dispatch::unmatch_mods", |b| {
        let mut keybinds = keybinds.clone();
        b.iter(|| {
            assert!(keybinds
                .dispatch(KeyInput::new('w', Mods::CTRL | Mods::ALT))
                .is_none());
        })
    });
}

criterion_group!(bench, parse, dispatch);
criterion_main!(bench);

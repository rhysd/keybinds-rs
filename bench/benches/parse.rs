use criterion::{black_box, criterion_group, criterion_main, Criterion};
use keybinds::KeySeq;
use std::str::FromStr;

fn parse_key_seq(c: &mut Criterion) {
    c.bench_function("single_char", |b| {
        b.iter(|| black_box(KeySeq::from_str("a").unwrap()))
    });
    c.bench_function("multiple_chars", |b| {
        b.iter(|| black_box(KeySeq::from_str("h e l l o").unwrap()))
    });
    c.bench_function("single_char_with_mod", |b| {
        b.iter(|| black_box(KeySeq::from_str("Ctrl+A").unwrap()))
    });
    c.bench_function("multiple_chars_with_mod", |b| {
        b.iter(|| black_box(KeySeq::from_str("Ctrl+H Shift+E Mod+L Alt+L Super+O").unwrap()))
    });
    c.bench_function("single_named_char", |b| {
        b.iter(|| black_box(KeySeq::from_str("Tab").unwrap()))
    });
    c.bench_function("multiple_named_chars", |b| {
        b.iter(|| black_box(KeySeq::from_str("Up Up Down Down Left Right Left Right B A").unwrap()))
    });
}

criterion_group!(parse, parse_key_seq);
criterion_main!(parse);

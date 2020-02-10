#![feature(test)]

extern crate test;
extern crate conway_gol;
use conway_gol::Universe;

#[bench]
fn universe_ticks(b: &mut test::Bencher) {
    let mut universe = Universe::new(256, 256);

    b.iter(|| {
        universe.tick();
    });
}

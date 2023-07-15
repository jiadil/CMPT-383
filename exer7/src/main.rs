pub mod find;
pub mod hailstone;
pub mod rational;
use hailstone::hailstone_sequence_prealloc;

fn main() {
    // nothing is required here, but you may want to use it for testing.
    println!("Hello, world!");
    println!("{:?}", hailstone_sequence_prealloc(5));
}

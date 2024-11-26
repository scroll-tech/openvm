use intermediate::sample_func;
use test_macro::declare;

declare!(Baz, 4);

fn main() {
    sample_func();
    let z = Baz;
    z.print();
}

use intermediate::sample_func;
use test_macro::declare;

declare!(Baz);

fn main() {
    sample_func();
    let z = Baz;
    z.print();
}

use intermediate::sample_func;
use test_macro::{declare, define};

declare!(Baz, 4);

define!(2, 3, 4);

fn main() {
    sample_func();
    let z = Baz;
    z.print_name();
    z.print_num();
}

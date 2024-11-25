use intermediate::sample_func;
use test_macro::declare;

fn main() {
    sample_func();
    declare!(j);
    while j_0 < 2 {
        println!("{}", j_0);
        j_0 += 1;
    }
}

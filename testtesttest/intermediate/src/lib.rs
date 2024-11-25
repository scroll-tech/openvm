use test_macro::declare;

pub fn sample_func() {
    declare!(i);
    declare!(i);
    while i_0 < 10 {
        while i_1 < i_0 {
            println!("{}, {}", i_0, i_1);
            i_1 += 1;
        }
        i_0 += 1;
    }
}

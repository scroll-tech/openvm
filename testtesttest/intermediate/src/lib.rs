use test_macro::declare;

declare!(Foo, 2);
declare!(Bar, 3);

pub fn sample_func() {
    let x = Foo;
    let y = Bar;
    x.print();
    y.print();
}

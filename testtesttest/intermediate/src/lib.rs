use test_macro::declare;

declare!(Foo);
declare!(Bar);

pub fn sample_func() {
    let x = Foo;
    let y = Bar;
    x.print();
    y.print();
}

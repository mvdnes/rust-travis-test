extern crate mytest;

#[test]
fn get() {
    let f = mytest::Foo::new();
    assert_eq!(f.get(), 42);
}

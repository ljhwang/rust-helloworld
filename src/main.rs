fn main() {
    println!("Hello, world!");
}

#[test]
fn simple_math() {
    assert_eq!(2 + 2, 4);
}

#[test]
#[should_panic(expected = "assertion failed")]
fn newspeak_math() {
    assert_eq!(2 + 2, 5);
}

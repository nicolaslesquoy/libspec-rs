extern crate libspec_rs;

use libspec_rs::add;

#[test]
fn it_works() {
    let result = add(2, 2);
    assert_eq!(result, 4);
}
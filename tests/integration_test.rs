use learn_rust;
use learn_rust::Rectangle;

#[test]
fn can_hold() {
    assert!(Rectangle::new(3, 1).can_hold(&Rectangle::new(0, 0)));
}

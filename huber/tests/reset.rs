use crate::common::reset_huber;

#[macro_use]
mod common;

#[test]
fn test_reset() {
    let assert = reset_huber();
    assert_eq_last_line!(assert.get_output().stderr, "[INFO ] Huber reset");
}

use sequential_test::sequential;

use crate::common::reset_huber;

#[macro_use]
mod common;

#[test]
#[sequential]
fn test_reset() {
    let assert = reset_huber();
    assert_contain_line_regex!(assert.get_output().stderr, "Huber reset");
}

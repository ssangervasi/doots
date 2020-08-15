use spectral::assert_that;

use doots::game::basic_types::{dot, edge, DotBox};

#[test]
pub fn test_dot_box_edges() {
    assert_that!(DotBox(dot(3, 7)).edges()).is_equal_to(vec![
        edge((3, 7), (3, 8)),
        edge((3, 8), (4, 8)),
        edge((4, 7), (4, 8)),
        edge((3, 7), (4, 7)),
    ])
}
